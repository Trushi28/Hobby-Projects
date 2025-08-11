# COSMIC DRIFT
# A real-time multiplayer space racing game with terminal UI and chat
import os
import sys
import time
import socket
import threading
import random
import json
import struct # For message framing
from queue import Queue
import argparse
from datetime import datetime

# Try importing blessed for cross-platform terminal UI
try:
    from blessed import Terminal
except ImportError:
    print("Error: 'blessed' library not found.")
    print("Please install it using: pip install blessed")
    sys.exit(1)

# Game settings
FPS = 20
TRACK_LENGTH = 100
SERVER_PORT = 5555
MAX_PLAYERS = 8
SHIP_DESIGNS = [
    "<=o=>",    # Default ship
    "<-=+=>",   # Longer ship
    "<~o~>",    # Wave ship
    "<=◊=>",    # Diamond ship
    "<==*==>",  # Star ship
    "<-=o=->",  # Balanced ship
    "</|\\>",   # Arrow ship NOTE: Backslash needs escaping
    "<[O]>",    # Boxed ship
    "<!--⚡>",   # Lightning ship (NEW)
    "<≈≈O≈≈>",  # Hover ship (NEW) 
    "<⚔◯⚔>",   # Battle ship (NEW)
]
# Map color names to blessed colors (adjust if needed)
# Using basic ANSI color names supported by blessed
COLORS = {
    "red": "red",
    "green": "green",
    "yellow": "yellow",
    "blue": "blue",
    "magenta": "magenta",
    "cyan": "cyan",
    "white": "white",
    "black": "black" # Added black for background potentially
}
COLOR_NAMES = list(COLORS.keys())
OBSTACLE_CHARS = ["*", "#", "@", "&", "%", "$", "!", "?"]
MESSAGE_HEADER_FORMAT = "!I" # Network byte order, unsigned integer (4 bytes)
MESSAGE_HEADER_SIZE = struct.calcsize(MESSAGE_HEADER_FORMAT)

# ASCII Art
TITLE_ART = """
╔═══════════════════════════════════════════════════════╗
║   ██████╗ ██████╗ ███████╗███╗   ███╗██╗ ██████╗      ║
║  ██╔════╝██╔═══██╗██╔════╝████╗ ████║██║██╔════╝      ║
║  ██║     ██║   ██║███████╗██╔████╔██║██║██║           ║
║  ██║     ██║   ██║╚════██║██║╚██╔╝██║██║██║           ║
║  ╚██████╗╚██████╔╝███████║██║ ╚═╝ ██║██║╚██████╗      ║
║   ╚═════╝ ╚═════╝ ╚══════╝╚═╝     ╚═╝╚═╝ ╚═════╝      ║
║                                                       ║
║  ██████╗ ██████╗ ██╗███████╗████████╗                 ║
║  ██╔══██╗██╔══██╗██║██╔════╝╚══██╔══╝                 ║
║  ██║  ██║██████╔╝██║█████╗     ██║                    ║
║  ██║  ██║██╔══██╗██║██╔══╝     ██║                    ║
║  ██████╔╝██║  ██║██║██║        ██║                    ║
║  ╚═════╝ ╚═╝  ╚═╝╚═╝╚═╝        ╚═╝                    ║
╚═══════════════════════════════════════════════════════╝
"""

EXPLOSION_ART = [
    "   * ",
    " * * *",
    "* * * ",
    " * * *",
    "   * "
]

# --- Utility Functions ---
def send_message(sock, message_dict):
    """Encodes, prefixes with length, and sends a message dictionary."""
    try:
        json_data = json.dumps(message_dict).encode('utf-8')
        message_len = len(json_data)
        header = struct.pack(MESSAGE_HEADER_FORMAT, message_len)
        sock.sendall(header + json_data)
        return True
    except (socket.error, BrokenPipeError, struct.error) as e:
        # print(f"Debug: Error sending message: {e}") # Optional debug
        return False
    except Exception as e:
        print(f"Unexpected error sending message: {e}")
        return False

def receive_message(sock):
    """Receives a length-prefixed message and decodes it."""
    try:
        # Read the header to get the message length
        header_data = sock.recv(MESSAGE_HEADER_SIZE)
        if not header_data or len(header_data) < MESSAGE_HEADER_SIZE:
            # print("Debug: Connection closed or incomplete header received.") # Optional debug
            return None # Connection closed or error

        message_len = struct.unpack(MESSAGE_HEADER_FORMAT, header_data)[0]

        # Read the full message payload
        chunks = []
        bytes_received = 0
        while bytes_received < message_len:
            chunk = sock.recv(min(message_len - bytes_received, 4096))
            if not chunk:
                # print("Debug: Connection closed while receiving payload.") # Optional debug
                return None # Connection closed prematurely
            chunks.append(chunk)
            bytes_received += len(chunk)

        json_data = b''.join(chunks).decode('utf-8')
        return json.loads(json_data)

    except (socket.error, ConnectionResetError, struct.error, json.JSONDecodeError) as e:
        # print(f"Debug: Error receiving message: {e}") # Optional debug
        return None
    except Exception as e:
        print(f"Unexpected error receiving message: {e}")
        return None

# --- Game State and Player Classes (Minor changes for consistency) ---
class GameState:
    def __init__(self):
        self.players = {} # {name: player_dict}
        self.obstacles = [] # [{"position": float, "lane": int, "type": str}]
        self.powerups = [] # [{"position": float, "lane": int, "type": str, "active": bool}]
        self.chat_messages = [] # [{"time": str, "sender": str, "message": str, "color": str}]
        self.game_started = False
        self.countdown = 3 # Seconds
        self.game_finished = False
        self.winner = None # Player name

    def to_dict(self):
        # Already stores players as dicts, just return internal state
        return {
            "players": self.players,
            "obstacles": self.obstacles,
            "powerups": self.powerups,
            # Limit chat history sent over network
            "chat_messages": self.chat_messages[-15:] if self.chat_messages else [],
            "game_started": self.game_started,
            "countdown": self.countdown,
            "game_finished": self.game_finished,
            "winner": self.winner
        }

    @classmethod
    def from_dict(cls, data):
        state = cls()
        # Directly assign if the structure matches
        state.players = data.get("players", {})
        state.obstacles = data.get("obstacles", [])
        state.powerups = data.get("powerups", [])
        state.chat_messages = data.get("chat_messages", [])
        state.game_started = data.get("game_started", False)
        state.countdown = data.get("countdown", 0)
        state.game_finished = data.get("game_finished", False)
        state.winner = data.get("winner", None)
        return state

class Player:
    # Represents player state, used mainly for logic, stored as dict in GameState
    def __init__(self, name, ship_design, color):
        self.name = name
        self.position = 0.0
        self.lane = random.randint(0, 4)
        self.speed = 1.0 # Base speed units per second
        self.boost_timer = 0.0 # Seconds remaining for boost
        self.ship_design = ship_design
        self.color = color # Color name string
        self.finished = False
        self.finish_time = None # Timestamp
        self.crashed = False
        self.crash_timeout = 0.0 # Seconds remaining until recovery

    def to_dict(self):
        return {
            "name": self.name,
            "position": self.position,
            "lane": self.lane,
            "speed": self.speed,
            "boost_timer": self.boost_timer,
            "ship_design": self.ship_design,
            "color": self.color,
            "finished": self.finished,
            "finish_time": self.finish_time,
            "crashed": self.crashed,
            "crash_timeout": self.crash_timeout
        }

    @classmethod
    def from_dict(cls, data):
        # Create a Player instance from a dictionary (useful for server-side logic)
        player = cls(data["name"], data["ship_design"], data["color"])
        player.position = data.get("position", 0.0)
        player.lane = data.get("lane", 0)
        player.speed = data.get("speed", 1.0)
        player.boost_timer = data.get("boost_timer", 0.0)
        player.finished = data.get("finished", False)
        player.finish_time = data.get("finish_time", None)
        player.crashed = data.get("crashed", False)
        player.crash_timeout = data.get("crash_timeout", 0.0)
        return player

# --- Game Server ---
class GameServer:
    def __init__(self, port=SERVER_PORT):
        self.port = port
        self.server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        self.clients = {} # {player_name: client_socket}
        self.game_state = GameState()
        self.lock = threading.Lock() # Protect shared game_state and clients
        self.running = False
        self.last_update_time = time.time()

    def start(self):
        try:
            self.server_socket.bind(('0.0.0.0', self.port))
            self.server_socket.listen(MAX_PLAYERS)
            self.running = True
            print(f"Server started on port {self.port}. Waiting for players...")
            self.generate_course()
            threading.Thread(target=self.game_loop, daemon=True).start()

            while self.running:
                try:
                    client_socket, address = self.server_socket.accept()
                    print(f"Connection from {address}")
                    threading.Thread(target=self.handle_client, args=(client_socket, address), daemon=True).start()
                except OSError as e: # Handle socket closing while accepting
                     if self.running:
                         print(f"Error accepting connection: {e}")
                     break # Exit accept loop if server stopped
        except Exception as e:
            print(f"Server failed to start: {e}")
        finally:
            print("Server shutting down...")
            self.running = False
            # Close all client sockets
            with self.lock:
                # Use list to avoid RuntimeError: dictionary changed size during iteration
                client_sockets = list(self.clients.values())
                for sock in client_sockets:
                    try:
                        sock.close()
                    except socket.error:
                        pass # Ignore errors if already closed
            self.server_socket.close()

    def generate_course(self):
        self.game_state.obstacles = []
        num_obstacles = random.randint(20, 35)
        for _ in range(num_obstacles):
            position = random.uniform(15, TRACK_LENGTH - 10) # Avoid start/end
            lane = random.randint(0, 4)
            obstacle_type = random.choice(OBSTACLE_CHARS)
            # Ensure no obstacles too close together in the same lane
            too_close = any(
                o["lane"] == lane and abs(o["position"] - position) < 5
                for o in self.game_state.obstacles
            )
            if not too_close:
                self.game_state.obstacles.append({"position": position, "lane": lane, "type": obstacle_type})

        self.game_state.powerups = []
        num_powerups = random.randint(5, 12)
        for _ in range(num_powerups):
            position = random.uniform(10, TRACK_LENGTH - 15)
            lane = random.randint(0, 4)
            # Ensure no powerups too close or overlapping obstacles
            too_close = any(
                (p["lane"] == lane and abs(p["position"] - position) < 5)
                or (o["lane"] == lane and abs(o["position"] - position) < 3)
                for p in self.game_state.powerups for o in self.game_state.obstacles
            )
            if not too_close:
                 self.game_state.powerups.append({"position": position, "lane": lane, "type": "boost", "active": True})


    def handle_client(self, client_socket, address):
        player_name = None
        try:
            # Initial handshake: receive player info
            client_socket.settimeout(10) # Timeout for initial info
            initial_data = receive_message(client_socket)
            client_socket.settimeout(None) # Reset timeout

            if not initial_data or "name" not in initial_data:
                print(f"Invalid initial data from {address}. Disconnecting.")
                client_socket.close()
                return

            with self.lock:
                if len(self.game_state.players) >= MAX_PLAYERS:
                    send_message(client_socket, {"error": "SERVER_FULL"})
                    client_socket.close()
                    return

                base_name = initial_data["name"]
                player_name = base_name
                counter = 1
                while player_name in self.game_state.players:
                    player_name = f"{base_name}_{counter}"
                    counter += 1

                ship_design = initial_data.get("ship_design", random.choice(SHIP_DESIGNS))
                color = initial_data.get("color", random.choice(COLOR_NAMES))

                # Validate inputs slightly
                if ship_design not in SHIP_DESIGNS: ship_design = SHIP_DESIGNS[0]
                if color not in COLOR_NAMES: color = COLOR_NAMES[0]

                player = Player(player_name, ship_design, color)
                self.game_state.players[player_name] = player.to_dict()
                self.clients[player_name] = client_socket

                self.add_chat_message("SERVER", f"{player_name} has joined the race!", "yellow")

                # Send confirmation and current game state
                send_message(client_socket, {"status": "JOINED", "your_name": player_name})
                send_message(client_socket, self.game_state.to_dict())

                # If this is the first player, notify they are host
                if len(self.game_state.players) == 1:
                    self.add_chat_message("SERVER", f"{player_name} is the host. Type !start to begin.", "cyan")


            # Listen for commands from this client
            while self.running:
                command = receive_message(client_socket)
                if command is None: # Handle disconnect or receive error
                    break # Exit loop to clean up

                with self.lock:
                    # Ensure player still exists (might be kicked?)
                    if player_name not in self.game_state.players:
                        break

                    if "chat" in command:
                        self.handle_chat(player_name, command["chat"])
                    elif "move" in command:
                        self.handle_move(player_name, command["move"])
                    # Start/Restart commands handled in chat now

        except (socket.timeout, ConnectionResetError, BrokenPipeError) as e:
            print(f"Network error with {player_name or address}: {e}")
        except Exception as e:
            print(f"Error handling client {player_name or address}: {e}")
        finally:
            # Cleanup client
            with self.lock:
                if player_name and player_name in self.clients:
                    del self.clients[player_name]
                if player_name and player_name in self.game_state.players:
                    # Don't remove player immediately if game is running, mark as disconnected?
                    # For simplicity now, just remove. Could add a "disconnected" flag later.
                    del self.game_state.players[player_name]
                    self.add_chat_message("SERVER", f"{player_name} has left the race.", "red")
            try:
                client_socket.close()
            except socket.error:
                pass # Ignore errors closing already closed socket
            print(f"Connection closed for {player_name or address}")

    def add_chat_message(self, sender, message, color):
        """Helper to add a chat message with timestamp."""
        self.game_state.chat_messages.append({
            "time": datetime.now().strftime("%H:%M:%S"),
            "sender": sender,
            "message": message,
            "color": color
        })
        # Optional: Limit total chat history size in memory
        if len(self.game_state.chat_messages) > 100:
            self.game_state.chat_messages.pop(0)


    def handle_chat(self, player_name, message):
        message = message.strip()
        if not message: return

        # Handle commands
        if message.startswith("!"):
            command = message[1:].lower()
            # Check if player exists before accessing keys
            if not self.game_state.players:
                return # No players, cannot determine host
            player_keys = list(self.game_state.players.keys())
            is_host = player_keys[0] == player_name if player_keys else False


            if command == "start" and not self.game_state.game_started:
                if is_host:
                    self.start_game()
                else:
                    self.add_chat_message("SERVER", "Only the host can start the race.", "red")
            elif command == "restart" and self.game_state.game_finished:
                 if is_host:
                     self.restart_game()
                 else:
                     self.add_chat_message("SERVER", "Only the host can restart the race.", "red")
            # Add other commands here if needed
            else:
                 # Avoid sending unknown command message if player list is empty during command processing
                 if player_name in self.game_state.players:
                    self.add_chat_message("SERVER", f"Unknown command: {command}", "red")

        else: # Regular chat message
            # Ensure player exists before sending chat
             if player_name in self.game_state.players:
                player_color = self.game_state.players[player_name].get("color", "white")
                self.add_chat_message(player_name, message, player_color)

    def handle_move(self, player_name, move_direction):
        # Only allow moves if game is running and player is not finished/crashed
        if not self.game_state.game_started or self.game_state.game_finished: return

        player_data = self.game_state.players.get(player_name)
        if not player_data or player_data["finished"] or player_data["crashed"]: return

        current_lane = player_data["lane"]
        if move_direction == "up" and current_lane > 0:
            player_data["lane"] -= 1
        elif move_direction == "down" and current_lane < 4:
            player_data["lane"] += 1
        # No need to update self.game_state.players here, modifying dict directly

    def start_game(self):
        if len(self.game_state.players) < 1: # Need at least one player
             self.add_chat_message("SERVER", "Cannot start race without players.", "red")
             return
        self.game_state.game_started = True
        self.game_state.countdown = 3
        self.add_chat_message("SERVER", "Race starting in 3...", "green")

    def restart_game(self):
        # Reset player states
        for name in self.game_state.players:
            player_data = self.game_state.players[name]
            player_data["position"] = 0.0
            player_data["lane"] = random.randint(0, 4)
            player_data["speed"] = 1.0
            player_data["boost_timer"] = 0.0
            player_data["finished"] = False
            player_data["finish_time"] = None
            player_data["crashed"] = False
            player_data["crash_timeout"] = 0.0

        self.generate_course() # Generate new obstacles/powerups

        self.game_state.game_finished = False
        self.game_state.winner = None
        self.game_state.game_started = True # Start immediately into countdown
        self.game_state.countdown = 3
        self.add_chat_message("SERVER", "New race starting in 3...", "green")


    def check_collisions_and_powerups(self, player_name, player_data):
        # Check collisions only if player is active
        if player_data["crashed"] or player_data["finished"]:
            return

        # Check obstacle collisions
        for obstacle in self.game_state.obstacles:
            if (player_data["lane"] == obstacle["lane"] and
                abs(player_data["position"] - obstacle["position"]) < 1.5): # Collision threshold
                player_data["crashed"] = True
                player_data["crash_timeout"] = 3.0 # 3 second recovery
                player_data["speed"] = 0 # Stop immediately
                player_data["boost_timer"] = 0 # Lose boost on crash
                self.add_chat_message("SERVER", f"{player_name} crashed!", "red")
                return # Stop checking after one crash

        # Check powerup collisions (only if not crashed)
        for powerup in self.game_state.powerups:
            if (powerup.get("active", False) and # Check if key exists and is True
                player_data["lane"] == powerup.get("lane") and
                abs(player_data["position"] - powerup.get("position", -1000)) < 1.5): # Check position exists
                powerup["active"] = False # Consume powerup
                if powerup.get("type") == "boost":
                    player_data["boost_timer"] = 5.0 # Add 5 seconds of boost
                    # Speed increase is handled in update_player based on boost_timer
                    self.add_chat_message("SERVER", f"{player_name} got a speed boost!", "cyan")
                # Add other powerup types here
                break # Only collect one powerup per update cycle


    def update_player(self, player_name, player_data, dt):
        if player_data["finished"]:
            return

        # Handle crash recovery
        if player_data["crashed"]:
            player_data["crash_timeout"] -= dt
            if player_data["crash_timeout"] <= 0:
                player_data["crashed"] = False
                player_data["crash_timeout"] = 0
                player_data["speed"] = 1.0 # Reset to base speed
                self.add_chat_message("SERVER", f"{player_name} recovered.", "green")
            else:
                return # Still crashed, no movement

        # Handle boost
        current_speed = 1.0 # Base speed
        if player_data["boost_timer"] > 0:
            player_data["boost_timer"] -= dt
            if player_data["boost_timer"] <= 0:
                player_data["boost_timer"] = 0
                # Speed drops back to normal (no message needed)
            else:
                current_speed = 1.75 # Boost speed multiplier

        player_data["speed"] = current_speed # Update speed field for display/logic

        # Update position
        player_data["position"] += player_data["speed"] * dt * 10 # Adjust multiplier for game feel

        # Check for finish
        if player_data["position"] >= TRACK_LENGTH:
            player_data["position"] = TRACK_LENGTH
            if not player_data["finished"]: # Check finished flag to avoid multiple messages
                player_data["finished"] = True
                player_data["finish_time"] = time.time()
                self.add_chat_message("SERVER", f"{player_name} finished!", "green")

                # Check for winner (first to finish)
                if self.game_state.winner is None:
                    self.game_state.winner = player_name
                    self.add_chat_message("SERVER", f"{player_name} WINS!", "yellow")


    def check_game_end(self):
        """Checks if all players have finished."""
        if self.game_state.game_finished or not self.game_state.game_started:
            return False # Already finished or not started

        if not self.game_state.players: # No players left
             self.game_state.game_finished = True
             self.game_state.game_started = False # Stop the game logic
             self.add_chat_message("SERVER", "All players left. Game over.", "yellow")
             return True


        all_finished = True
        for player_data in self.game_state.players.values():
            if not player_data["finished"]:
                all_finished = False
                break

        if all_finished:
            self.game_state.game_finished = True
            self.add_chat_message("SERVER", "Race finished! Host can type !restart", "cyan")
            return True
        return False

    def broadcast_state(self):
        """Sends the current game state to all connected clients."""
        state_dict = self.game_state.to_dict()
        disconnected_players = []

        # Iterate over a copy of items in case clients disconnect during iteration
        client_items = list(self.clients.items())

        for player_name, client_socket in client_items:
            if not send_message(client_socket, state_dict):
                # print(f"Debug: Failed to send state to {player_name}. Marking for removal.") # Optional debug
                disconnected_players.append(player_name)

        # Clean up disconnected clients *after* iterating
        if disconnected_players:
            # print(f"Debug: Cleaning up disconnected players: {disconnected_players}") # Optional debug
            for name in disconnected_players:
                if name in self.clients:
                    try:
                        self.clients[name].close() # Attempt to close socket
                    except socket.error:
                        pass # Ignore errors if already closed
                    del self.clients[name]
                # Player data is removed in handle_client's finally block now
                # Optionally, mark player as disconnected in game_state instead of removing immediately

    def game_loop(self):
        """Main server loop to update game logic and broadcast state."""
        countdown_timer = 0.0

        while self.running:
            current_time = time.time()
            # Prevent large dt spikes on resume/lag
            dt = min(current_time - self.last_update_time, 1.0 / FPS * 5) # Max dt = 5 frames
            self.last_update_time = current_time

            with self.lock:
                if self.game_state.game_started:
                    # Handle countdown logic
                    if self.game_state.countdown > 0:
                        countdown_timer += dt
                        if countdown_timer >= 1.0:
                            self.game_state.countdown -= 1
                            countdown_timer = 0.0 # Reset timer for next second
                            if self.game_state.countdown > 0:
                                self.add_chat_message("SERVER", f"{self.game_state.countdown}...", "green")
                            else:
                                self.add_chat_message("SERVER", "GO!", "green")
                    # Update game only after countdown finishes
                    elif not self.game_state.game_finished:
                        player_names = list(self.game_state.players.keys()) # Iterate over copy
                        for name in player_names:
                             if name in self.game_state.players: # Check if player still exists
                                player_data = self.game_state.players[name]
                                self.check_collisions_and_powerups(name, player_data)
                                self.update_player(name, player_data, dt)

                        self.check_game_end()

                # Always broadcast the latest state
                self.broadcast_state()

            # Control loop speed
            sleep_time = (1.0 / FPS) - (time.time() - current_time)
            if sleep_time > 0:
                time.sleep(sleep_time)


# --- Game Client ---
class GameClient:
    def __init__(self, server_ip, port=SERVER_PORT):
        self.server_ip = server_ip
        self.port = port
        self.client_socket = None
        self.game_state = None
        self.player_name = None # Set by server on successful join
        self.running = False
        self.lock = threading.Lock() # Protect shared game_state
        self.term = Terminal() # Blessed terminal instance
        self.last_received_state = None # Store the actual dict

    def connect(self, player_name_req, ship_design, color):
        try:
            self.client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            # Set a timeout for the connection attempt itself
            self.client_socket.settimeout(10)
            self.client_socket.connect((self.server_ip, self.port))
            self.client_socket.settimeout(None) # Reset timeout after connection

            # Send initial player info request
            initial_info = {
                "name": player_name_req,
                "ship_design": ship_design,
                "color": color
            }
            if not send_message(self.client_socket, initial_info):
                 print("Failed to send initial info to server.")
                 self.client_socket.close()
                 return False

            # Wait for JOINED confirmation and initial state
            print("Waiting for server confirmation...")
            # Set timeout for receiving confirmation
            self.client_socket.settimeout(10)
            confirmation = receive_message(self.client_socket)
            self.client_socket.settimeout(None) # Reset timeout

            if not confirmation or confirmation.get("status") != "JOINED":
                 error_msg = confirmation.get("error", "Unknown reason") if confirmation else "No response or timeout"
                 print(f"Server rejected connection or invalid response: {error_msg}")
                 self.client_socket.close()
                 return False

            self.player_name = confirmation.get("your_name", player_name_req) # Use name server assigned
            print(f"Successfully joined as '{self.player_name}'. Waiting for game state...")

            # Receive the first full game state
            # Set timeout for receiving initial state
            self.client_socket.settimeout(10)
            initial_state_dict = receive_message(self.client_socket)
            self.client_socket.settimeout(None) # Reset timeout

            if not initial_state_dict:
                 print("Failed to receive initial game state (timeout or error).")
                 self.client_socket.close()
                 return False

            with self.lock:
                self.last_received_state = initial_state_dict
                self.game_state = GameState.from_dict(initial_state_dict)

            return True

        except socket.timeout:
             print(f"Error: Connection or initial communication timed out with {self.server_ip}:{self.port}.")
             if self.client_socket: self.client_socket.close()
             return False
        except socket.gaierror:
             print(f"Error: Could not resolve hostname '{self.server_ip}'.")
             return False
        except ConnectionRefusedError:
            print(f"Error: Connection refused by server at {self.server_ip}:{self.port}. Is the server running?")
            return False
        except Exception as e:
            print(f"Error connecting to server: {e}")
            if self.client_socket:
                self.client_socket.close()
            return False

    def start(self):
        self.running = True
        # Start listener thread first
        listener_thread = threading.Thread(target=self.listen_for_updates, daemon=True)
        listener_thread.start()

        # Run UI in the main thread using blessed context manager
        try:
            # Use blessed context manager for clean terminal handling
            with self.term.fullscreen(), self.term.cbreak(), self.term.hidden_cursor():
                 self.run_ui()
        except KeyboardInterrupt:
            print("Exiting...")
        except Exception as e:
            # Ensure terminal state is restored even on error
            # Use term.normal to reset terminal state before printing error
            print(self.term.normal + f"\nAn error occurred in the UI loop: {e}")
            import traceback
            traceback.print_exc() # Print full traceback for debugging
        finally:
            self.running = False
            if self.client_socket:
                try:
                    # Try shutting down read/write ends first
                    self.client_socket.shutdown(socket.SHUT_RDWR)
                except (socket.error, OSError):
                     pass # Ignore errors if already closed or not connected
                finally:
                     try:
                         self.client_socket.close()
                     except socket.error:
                          pass # Ignore errors if already closed
            print("Disconnected.")
            # Blessed context manager handles terminal cleanup automatically

    def listen_for_updates(self):
        while self.running:
            state_dict = receive_message(self.client_socket)
            if state_dict is None:
                if self.running: # Avoid error message if we are shutting down
                    # Use term.normal before printing to ensure visibility
                    print(self.term.normal + "\nConnection lost to server.")
                self.running = False # Signal UI thread to stop
                break

            with self.lock:
                self.last_received_state = state_dict # Store raw dict
                # Update GameState object (optional, if needed for client-side logic)
                # self.game_state = GameState.from_dict(state_dict)
        # print("Listener thread stopped.") # Optional debug


    def send_command(self, command):
        if not self.running: return
        if not send_message(self.client_socket, command):
            # Use term.normal before printing to ensure visibility if UI is active
            print(self.term.normal + "\nFailed to send command. Connection may be lost.")
            self.running = False # Stop client if send fails

    def run_ui(self):
        """Main UI loop using blessed."""
        term = self.term
        input_buffer = ""
        chat_mode = False

        while self.running:
            # Get latest state within lock
            with self.lock:
                current_state_dict = self.last_received_state
                if not current_state_dict:
                    # Draw loading/error message if state not available
                    # Use print directly with blessed formatting
                    print(term.clear + term.move_xy(0,0) + term.yellow("Waiting for server state..."), end='', flush=True)
                    time.sleep(0.1)
                    continue # Skip drawing rest of UI

                players = current_state_dict.get("players", {})
                obstacles = current_state_dict.get("obstacles", [])
                powerups = current_state_dict.get("powerups", [])
                chat_messages = current_state_dict.get("chat_messages", [])
                game_started = current_state_dict.get("game_started", False)
                countdown = current_state_dict.get("countdown", 0)
                game_finished = current_state_dict.get("game_finished", False)
                winner = current_state_dict.get("winner", None)


            # --- Drawing Logic ---
            # Use a buffer to build the screen output, then print once
            output_buffer = term.clear # Start with clear screen escape code

            # Get terminal size
            max_y, max_x = term.height, term.width

            # 1. Draw Title (Centered)
            title_lines = TITLE_ART.strip().split('\n')
            title_height = len(title_lines)
            title_width = max(len(term.strip_seqs(line)) for line in title_lines)
            start_y = 1
            start_x = max(0, (max_x - title_width) // 2)
            # Apply formatting directly
            for i, line in enumerate(title_lines):
                 if start_y + i < max_y: # Check bounds
                    output_buffer += term.move(start_y + i, start_x) + term.cyan(line)


            # 2. Draw Race Track Area
            track_start_y = start_y + title_height + 1
            track_height = 5 * 2 # 5 lanes, 1 space between
            track_view_width = max_x - 4 # Leave margin

            # Draw lanes
            for lane in range(5):
                lane_y = track_start_y + lane * 2
                if lane_y < max_y - 6: # Ensure space for progress/chat/status
                    output_buffer += term.move(lane_y, 2) + term.white("=" * track_view_width)

            # Draw obstacles within the view
            for obs in obstacles:
                obs_x = 2 + int((obs["position"] / TRACK_LENGTH) * track_view_width)
                obs_y = track_start_y + obs["lane"] * 2
                if 2 <= obs_x < max_x - 2 and track_start_y <= obs_y < track_start_y + track_height:
                     # Apply formatting directly
                     output_buffer += term.move(obs_y, obs_x) + term.red(obs["type"])

            # Draw active powerups
            for pwp in powerups:
                 if pwp.get("active", False):
                     pwp_x = 2 + int((pwp["position"] / TRACK_LENGTH) * track_view_width)
                     pwp_y = track_start_y + pwp["lane"] * 2
                     if 2 <= pwp_x < max_x - 2 and track_start_y <= pwp_y < track_start_y + track_height:
                          # Apply formatting directly
                          output_buffer += term.move(pwp_y, pwp_x) + term.green("+")

            # Draw players
            for p_name, p_data in players.items():
                p_x_track = p_data.get("position", 0)
                p_lane = p_data.get("lane", 0)
                p_color_name = p_data.get("color", "white")
                p_ship = p_data.get("ship_design", "?")
                p_crashed = p_data.get("crashed", False)
                p_color_func = getattr(term, p_color_name, term.white) # Get blessed color function

                p_screen_x = 2 + int((p_x_track / TRACK_LENGTH) * track_view_width)
                p_screen_y = track_start_y + p_lane * 2

                # Clamp screen coordinates to be within drawable area
                p_screen_x = max(2, min(p_screen_x, max_x - 3))
                p_screen_y = max(track_start_y, min(p_screen_y, track_start_y + track_height -1))


                if track_start_y <= p_screen_y < track_start_y + track_height:
                    if p_crashed:
                        # Draw simple explosion centered on player pos
                        # Apply formatting directly
                        if p_screen_y >=0 and p_screen_x >=0: output_buffer += term.move(p_screen_y, p_screen_x) + term.red_bold("*")
                        if p_screen_y-1 >=0 and p_screen_x-1 >=0: output_buffer += term.move(p_screen_y-1, p_screen_x-1) + term.yellow_bold("*")
                        if p_screen_y+1 < max_y and p_screen_x-1 >=0: output_buffer += term.move(p_screen_y+1, p_screen_x-1) + term.yellow_bold("*")
                        if p_screen_y-1 >=0 and p_screen_x+1 < max_x: output_buffer += term.move(p_screen_y-1, p_screen_x+1) + term.yellow_bold("*")
                        if p_screen_y+1 < max_y and p_screen_x+1 < max_x: output_buffer += term.move(p_screen_y+1, p_screen_x+1) + term.yellow_bold("*")

                    else:
                        # Draw ship design, centered roughly
                        ship_draw_x = max(2, p_screen_x - len(p_ship) // 2)
                        # Ensure ship doesn't overwrite boundary chars or go off screen
                        ship_draw_x = min(ship_draw_x, max_x - 2 - len(p_ship))
                        ship_draw_x = max(2, ship_draw_x)

                        # Apply formatting directly
                        output_buffer += term.move(p_screen_y, ship_draw_x) + p_color_func(term.bold(p_ship))

                    # Draw player name above ship (if space)
                    if p_screen_y > track_start_y:
                        name_draw_x = max(2, p_screen_x - len(p_name) // 2)
                        # Ensure name doesn't overwrite boundary chars or go off screen
                        name_draw_x = min(name_draw_x, max_x - 2 - len(p_name))
                        name_draw_x = max(2, name_draw_x)
                        # Apply formatting directly
                        output_buffer += term.move(p_screen_y - 1, name_draw_x) + p_color_func(p_name)


            # 3. Draw Progress Bar Area
            progress_y = max_y - 6 # Position above chat/status
            if progress_y > track_start_y + track_height: # Ensure it doesn't overlap track
                output_buffer += term.move(progress_y, 2) + term.white("0" + "-" * (track_view_width - 2) + str(TRACK_LENGTH))
                for p_name, p_data in players.items():
                     p_x_track = p_data.get("position", 0)
                     p_color_name = p_data.get("color", "white")
                     p_color_func = getattr(term, p_color_name, term.white)
                     prog_x = 2 + int((p_x_track / TRACK_LENGTH) * track_view_width)
                     prog_x = max(2, min(prog_x, max_x - 3)) # Clamp within bounds
                     # Apply formatting directly
                     output_buffer += term.move(progress_y, prog_x) + p_color_func("|")


            # 4. Draw Game Status Messages (Countdown, Winner, etc.)
            status_y = track_start_y + track_height + 1 # Below track
            if status_y < progress_y: # Ensure it doesn't overlap progress bar
                status_msg = ""
                status_color_func = term.white
                if game_started and countdown > 0:
                    status_msg = str(countdown)
                    status_color_func = term.yellow_bold
                elif game_finished and winner:
                    status_msg = f"{winner} WINS!"
                    status_color_func = term.green_bold
                elif game_finished:
                     status_msg = "Race Finished!"
                     status_color_func = term.cyan_bold
                # Removed the "GO!" message here, rely on chat

                if status_msg:
                    msg_len = len(term.strip_seqs(status_msg)) # Get length without escape codes
                    msg_x = max(0, (max_x - msg_len) // 2)
                    output_buffer += term.move(status_y, msg_x) + status_color_func(status_msg)


            # 5. Draw Status Line (Waiting, In Progress, etc.)
            status_line_y = max_y - 4
            my_player_data = players.get(self.player_name, {})
            status_text = ""
            if not game_started:
                status_text = "Waiting for host (!start)..."
            elif game_finished:
                status_text = "Race finished! Host can !restart"
            elif my_player_data.get("finished", False):
                 status_text = term.green("You finished!")
            elif my_player_data.get("crashed", False):
                 status_text = term.red("CRASHED! Recovering...")
            else:
                 status_text = "Race in progress! (↑/↓ move, 'c' chat, 'q' quit)"

            # Use ljust with width for proper clearing of the line
            output_buffer += term.move(status_line_y, 2) + term.ljust(status_text, width=max_x - 4)


            # 6. Draw Chat Area
            chat_start_y = max_y - 3
            chat_width = max_x - 4
            # Corrected line 953: Use term.dim + string + term.normal
            output_buffer += term.move(chat_start_y, 2) + term.bold("CHAT") + term.dim + ("-" * (chat_width - 5)) + term.normal

            # Draw last 2-3 messages
            num_chat_lines = 2
            visible_messages = chat_messages[-(num_chat_lines):]
            for i, msg in enumerate(visible_messages):
                line_y = chat_start_y + i + 1
                if line_y < max_y -1: # Ensure space for input prompt
                    time_str = f"[{msg.get('time', '??:??:??')}]"
                    sender = msg.get('sender', '???')
                    message = msg.get('message', '')
                    color_name = msg.get('color', 'white')
                    sender_color_func = getattr(term, color_name, term.white)

                    # Format: [TIME] SENDER: MESSAGE
                    # Apply formatting directly
                    # Corrected line 969: Use term.dim + string + term.normal
                    full_line = f"{term.dim}{time_str}{term.normal} {sender_color_func(sender + ':')} {message}"

                    # Truncate based on visible length
                    max_len = max_x - 4
                    # Use term.ljust for clearing rest of line, strip sequences for length calculation
                    visible_line = term.ljust(full_line, width=max_len)

                    output_buffer += term.move(line_y, 2) + visible_line


            # 7. Draw Input Prompt
            input_y = max_y - 1
            if chat_mode:
                prompt = "Chat: "
                # Apply formatting directly, add cursor manually
                output_buffer += term.move(chat_start_y, 2) + term.bold("CHAT") + term.dim + ("-" * (chat_width - 5)) + term.normal
            else:
                prompt = "Cmd: " # Keep it short
                 # Corrected line 984: Use term.dim + string + term.normal
                output_buffer += term.move(input_y, 2) + term.dim + prompt + term.normal

            # --- Final Print ---
            # Print the entire buffer at once to reduce flickering
            print(output_buffer, end='', flush=True)

            # --- Input Handling ---
            key = term.inkey(timeout=1.0/FPS) # Use FPS for timeout

            if key: # A key was pressed
                if chat_mode:
                    if key.is_sequence:
                        if key.name == "KEY_ENTER":
                            if input_buffer:
                                self.send_command({"chat": input_buffer})
                            input_buffer = ""
                            chat_mode = False
                        elif key.name == "KEY_BACKSPACE":
                            input_buffer = input_buffer[:-1]
                        elif key.name == "KEY_ESCAPE":
                            input_buffer = ""
                            chat_mode = False
                        # Handle other sequences if needed (e.g., delete, arrows within chat)
                    elif not key.is_sequence and key: # Regular printable character
                        input_buffer += key
                else: # Not in chat mode (command mode)
                    if key == 'c':
                        chat_mode = True
                        input_buffer = "" # Clear buffer when entering chat mode
                    elif key == 'q':
                        self.running = False # Signal exit
                        break # Exit UI loop immediately
                    elif key.is_sequence:
                        if key.name == "KEY_UP":
                            self.send_command({"move": "up"})
                        elif key.name == "KEY_DOWN":
                            self.send_command({"move": "down"})
                    # Allow sending commands directly without 'c' first
                    elif key == '!':
                         chat_mode = True # Enter chat mode to type command
                         input_buffer = "!"
                    elif key == 'r': # Shortcut for restart (if host)
                         self.send_command({"chat": "!restart"})
                    # Add other direct key commands if needed

            # No explicit sleep needed here as inkey(timeout) handles waiting

# --- Main Execution ---
def setup_player():
    """Get player information using simple input."""
    term = Terminal() # Temporary terminal for setup
    print(term.clear + term.cyan(TITLE_ART))
    print(term.bold("\nWelcome to COSMIC DRIFT!"))

    player_name = ""
    while not (3 <= len(player_name) <= 10):
        player_name = input(term.yellow("Enter your name (3-10 chars): ")).strip()

    print(term.green("\nChoose your ship design:"))
    for i, ship in enumerate(SHIP_DESIGNS):
        print(f"{i+1}. {ship}")
    ship_choice = 0
    while not (1 <= ship_choice <= len(SHIP_DESIGNS)):
        try:
            ship_choice = int(input(term.yellow(f"Enter number (1-{len(SHIP_DESIGNS)}): ")))
        except ValueError:
             print(term.red("Invalid input. Please enter a number."))

    print(term.green("\nChoose your ship color:"))
    for i, color in enumerate(COLOR_NAMES):
        # Use blessed for colored preview if possible
        color_func = getattr(term, color, term.white)
        print(f"{i+1}. {color_func(color)}")
    color_choice = 0
    while not (1 <= color_choice <= len(COLOR_NAMES)):
         try:
             color_choice = int(input(term.yellow(f"Enter number (1-{len(COLOR_NAMES)}): ")))
         except ValueError:
             print(term.red("Invalid input. Please enter a number."))

    return player_name, SHIP_DESIGNS[ship_choice - 1], COLOR_NAMES[color_choice - 1]

def main():
    parser = argparse.ArgumentParser(description="Cosmic Drift - Terminal Racer")
    parser.add_argument("--host", action="store_true", help="Host a game server")
    parser.add_argument("--connect", type=str, metavar="IP_ADDRESS", help="Connect to server IP")
    parser.add_argument("--port", type=int, default=SERVER_PORT, help=f"Port (default: {SERVER_PORT})")
    args = parser.parse_args()

    if args.host:
        try:
            server = GameServer(port=args.port)
            server.start() # This will block until server stops
        except KeyboardInterrupt:
            print('\nServer stopped manually')    
    elif args.connect:
        player_name, ship_design, color = setup_player()
        print(f"\nConnecting to {args.connect}:{args.port} as {player_name}...")
        client = GameClient(args.connect, port=args.port)
        if client.connect(player_name, ship_design, color):
            client.start() # Blocks until client exits
        else:
            print("Failed to connect.")
    else:
        # Default: Start local game (server in background thread, client in main)
        print("Starting local game...")
        server_thread = threading.Thread(target=lambda: GameServer(port=args.port).start(), daemon=True)
        server_thread.start()
        print("Server thread started. Waiting a moment...")
        time.sleep(2) # Give server time to bind port

        player_name, ship_design, color = setup_player()
        print(f"\nConnecting to localhost:{args.port} as {player_name}...")
        client = GameClient("127.0.0.1", port=args.port) # Explicitly use 127.0.0.1
        if client.connect(player_name, ship_design, color):
             client.start()
        else:
             print("Failed to connect to local server.")


if __name__ == "__main__":
    main()
