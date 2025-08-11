import random
import os
import time
from dataclasses import dataclass
from typing import List, Tuple, Dict, Optional

# Game constants
DUNGEON_WIDTH = 80
DUNGEON_HEIGHT = 24
FOV_RADIUS = 5

# Characters for map rendering
WALL = '#'
FLOOR = '.'
PLAYER = '@'
ENEMY = 'M'
HEALTH_POTION = '!'
WEAPON = '/'
STAIRS = '>'
FOG = ' '

# Colors (ANSI escape codes)
class Color:
    RESET = '\033[0m'
    PLAYER = '\033[1;32m'  # Bold Green
    ENEMY = '\033[1;31m'   # Bold Red
    WALL = '\033[1;30m'    # Bold Black
    FLOOR = '\033[0;37m'   # White
    HEALTH_POTION = '\033[1;35m'  # Bold Magenta
    WEAPON = '\033[1;34m'  # Bold Blue
    STAIRS = '\033[1;33m'  # Bold Yellow
    MESSAGE = '\033[1;36m'  # Bold Cyan

@dataclass
class Entity:
    x: int
    y: int
    char: str
    name: str
    color: str = Color.RESET
    hp: int = 0
    max_hp: int = 0
    attack: int = 0
    defense: int = 0
    xp: int = 0
    inventory: List[str] = None
    
    def __post_init__(self):
        if self.inventory is None:
            self.inventory = []

@dataclass
class Room:
    x: int
    y: int
    width: int
    height: int
    
    @property
    def center(self) -> Tuple[int, int]:
        center_x = int(self.x + self.width / 2)
        center_y = int(self.y + self.height / 2)
        return (center_x, center_y)
    
    @property
    def inner_area(self) -> List[Tuple[int, int]]:
        """Returns all floor coordinates within the room"""
        area = []
        for x in range(self.x + 1, self.x + self.width - 1):
            for y in range(self.y + 1, self.y + self.height - 1):
                area.append((x, y))
        return area
    
    def intersects(self, other: 'Room') -> bool:
        """Returns True if this room overlaps with another room"""
        return (
            self.x <= other.x + other.width and
            self.x + self.width >= other.x and
            self.y <= other.y + other.height and
            self.y + self.height >= other.y
        )

class Game:
    def __init__(self):
        self.dungeon = {}
        self.rooms = []
        self.entities = []
        self.player = None
        self.level = 1
        self.message_log = []
        self.visible_tiles = set()
        self.explored_tiles = set()
        self.game_over = False
        
    def init_game(self):
        """Initialize a new game"""
        self.dungeon = {}
        self.rooms = []
        self.entities = []
        self.visible_tiles = set()
        self.explored_tiles = set()
        self.game_over = False
        
        # Create player
        self.player = Entity(0, 0, PLAYER, "Player", Color.PLAYER, 30, 30, 5, 2, 0)
        
        # Generate dungeon
        self.generate_dungeon()
        
        # Place player in the center of the first room
        start_x, start_y = self.rooms[0].center
        self.player.x, self.player.y = start_x, start_y
        
        # Add player to entities list
        self.entities.append(self.player)
        
        # Add a welcome message
        self.message_log = [f"{Color.MESSAGE}Welcome to Dungeon Crawler! Level {self.level}{Color.RESET}"]
        
    def generate_dungeon(self):
        """Generate a new dungeon level"""
        # Initialize dungeon with walls
        for x in range(DUNGEON_WIDTH):
            for y in range(DUNGEON_HEIGHT):
                self.dungeon[(x, y)] = WALL
        
        self.rooms = []
        
        # Create rooms
        max_rooms = 10
        min_room_size = 6
        max_room_size = 12
        
        for _ in range(max_rooms):
            # Random width and height
            width = random.randint(min_room_size, max_room_size)
            height = random.randint(min_room_size, max_room_size)
            
            # Random position
            x = random.randint(1, DUNGEON_WIDTH - width - 1)
            y = random.randint(1, DUNGEON_HEIGHT - height - 1)
            
            new_room = Room(x, y, width, height)
            
            # Check if the room intersects with any existing room
            room_intersects = False
            for other_room in self.rooms:
                if new_room.intersects(other_room):
                    room_intersects = True
                    break
            
            if not room_intersects:
                # This room is valid, carve it out
                self._create_room(new_room)
                
                # Connect to previous room
                if self.rooms:
                    prev_room = self.rooms[-1]
                    prev_x, prev_y = prev_room.center
                    curr_x, curr_y = new_room.center
                    
                    # Randomly choose horizontal-first or vertical-first corridor
                    if random.randint(0, 1) == 1:
                        # Horizontal then vertical
                        self._create_h_tunnel(prev_x, curr_x, prev_y)
                        self._create_v_tunnel(prev_y, curr_y, curr_x)
                    else:
                        # Vertical then horizontal
                        self._create_v_tunnel(prev_y, curr_y, prev_x)
                        self._create_h_tunnel(prev_x, curr_x, curr_y)
                
                # Add room to the list
                self.rooms.append(new_room)
        
        # Place enemies, items, and stairs
        self._place_entities()
        self._place_stairs()
    
    def _create_room(self, room):
        """Carve out a room in the dungeon"""
        for x in range(room.x, room.x + room.width):
            for y in range(room.y, room.y + room.height):
                if x == room.x or x == room.x + room.width - 1 or y == room.y or y == room.y + room.height - 1:
                    # Walls
                    self.dungeon[(x, y)] = WALL
                else:
                    # Floor
                    self.dungeon[(x, y)] = FLOOR
    
    def _create_h_tunnel(self, x1, x2, y):
        """Create a horizontal tunnel between two points"""
        for x in range(min(x1, x2), max(x1, x2) + 1):
            self.dungeon[(x, y)] = FLOOR
    
    def _create_v_tunnel(self, y1, y2, x):
        """Create a vertical tunnel between two points"""
        for y in range(min(y1, y2), max(y1, y2) + 1):
            self.dungeon[(x, y)] = FLOOR
    
    def _place_entities(self):
        """Place enemies and items in the dungeon"""
        # Skip the first room (player's starting room)
        for room in self.rooms[1:]:
            # Add enemies
            num_enemies = random.randint(0, 2 + self.level // 2)
            for _ in range(num_enemies):
                pos = random.choice(room.inner_area)
                if not self.is_blocked(*pos):
                    enemy_type = random.choice(["Goblin", "Orc", "Skeleton"])
                    
                    if enemy_type == "Goblin":
                        hp = 6 + self.level
                        attack = 3 + self.level // 3
                        defense = 1
                        xp = 2
                    elif enemy_type == "Orc":
                        hp = 10 + self.level * 2
                        attack = 4 + self.level // 2
                        defense = 2
                        xp = 5
                    else:  # Skeleton
                        hp = 8 + self.level
                        attack = 5 + self.level // 2
                        defense = 0
                        xp = 3
                    
                    enemy = Entity(pos[0], pos[1], ENEMY, enemy_type, Color.ENEMY, hp, hp, attack, defense, xp)
                    self.entities.append(enemy)
            
            # Add items
            if random.random() < 0.6:  # 60% chance for a health potion
                pos = random.choice(room.inner_area)
                if not self.is_blocked(*pos):
                    item = Entity(pos[0], pos[1], HEALTH_POTION, "Health Potion", Color.HEALTH_POTION)
                    self.entities.append(item)
            
            if random.random() < 0.3:  # 30% chance for a weapon
                pos = random.choice(room.inner_area)
                if not self.is_blocked(*pos):
                    weapon_type = random.choice(["Dagger", "Sword", "Axe"])
                    item = Entity(pos[0], pos[1], WEAPON, weapon_type, Color.WEAPON)
                    self.entities.append(item)
    
    def _place_stairs(self):
        """Place stairs to the next level"""
        # Place stairs in the last room
        last_room = self.rooms[-1]
        stairs_x, stairs_y = last_room.center
        
        # Make sure the stairs aren't placed on the player or an entity
        while self.is_blocked(stairs_x, stairs_y):
            stairs_x = random.randint(last_room.x + 1, last_room.x + last_room.width - 2)
            stairs_y = random.randint(last_room.y + 1, last_room.y + last_room.height - 2)
        
        self.dungeon[(stairs_x, stairs_y)] = STAIRS
    
    def is_blocked(self, x, y):
        """Check if a position is blocked by a wall or an entity"""
        if (x, y) not in self.dungeon or self.dungeon[(x, y)] == WALL:
            return True
        
        for entity in self.entities:
            if entity.x == x and entity.y == y:
                return True
        
        return False
    
    def get_entity_at(self, x, y):
        """Get an entity at a specific position"""
        for entity in self.entities:
            if entity.x == x and entity.y == y:
                return entity
        return None
    
    def move_player(self, dx, dy):
        """Move the player by the given amount"""
        new_x = self.player.x + dx
        new_y = self.player.y + dy
        
        # Check for entity at destination
        target = self.get_entity_at(new_x, new_y)
        
        if target:
            if target.char == ENEMY:
                self.attack(self.player, target)
            elif target.char == HEALTH_POTION:
                self.player.hp = min(self.player.max_hp, self.player.hp + 10)
                self.message_log.append(f"{Color.MESSAGE}You drink a health potion and recover 10 HP!{Color.RESET}")
                self.entities.remove(target)
                self.player.x = new_x
                self.player.y = new_y
            elif target.char == WEAPON:
                weapon_bonus = {"Dagger": 2, "Sword": 4, "Axe": 6}.get(target.name, 3)
                self.player.attack += weapon_bonus
                self.message_log.append(f"{Color.MESSAGE}You pick up a {target.name} (+{weapon_bonus} attack)!{Color.RESET}")
                self.player.inventory.append(target.name)
                self.entities.remove(target)
                self.player.x = new_x
                self.player.y = new_y
        elif not self.is_blocked(new_x, new_y):
            self.player.x = new_x
            self.player.y = new_y
            
            # Check for stairs
            if self.dungeon[(new_x, new_y)] == STAIRS:
                self.next_level()
    
    def attack(self, attacker, defender):
        """Handle combat between entities"""
        damage = max(0, attacker.attack - defender.defense)
        damage = max(1, damage)  # Always do at least 1 damage
        
        defender.hp -= damage
        
        if attacker is self.player:
            self.message_log.append(f"{Color.MESSAGE}You hit the {defender.name} for {damage} damage!{Color.RESET}")
        else:
            self.message_log.append(f"{Color.MESSAGE}The {attacker.name} hits you for {damage} damage!{Color.RESET}")
        
        if defender.hp <= 0:
            if defender is self.player:
                self.message_log.append(f"{Color.MESSAGE}You died! Game over.{Color.RESET}")
                self.game_over = True
            else:
                self.message_log.append(f"{Color.MESSAGE}You killed the {defender.name}! You gain {defender.xp} XP.{Color.RESET}")
                self.player.xp += defender.xp
                self.entities.remove(defender)
                
                # Check for level up
                if self.player.xp >= 10 * (self.player.max_hp - 29):  # Level up formula
                    self.player.max_hp += 5
                    self.player.hp += 5
                    self.player.attack += 1
                    self.player.defense += 1
                    self.message_log.append(f"{Color.MESSAGE}Level up! You feel stronger!{Color.RESET}")
    
    def process_enemy_turns(self):
        """Process turns for all enemies"""
        for entity in [e for e in self.entities if e.char == ENEMY]:
            # Simple AI: If player is nearby, move toward player
            dist_x = self.player.x - entity.x
            dist_y = self.player.y - entity.y
            distance = abs(dist_x) + abs(dist_y)
            
            if distance <= 5:  # Enemy can see the player
                dx = 0
                dy = 0
                
                if abs(dist_x) > abs(dist_y):
                    dx = 1 if dist_x > 0 else -1
                else:
                    dy = 1 if dist_y > 0 else -1
                
                new_x = entity.x + dx
                new_y = entity.y + dy
                
                if (new_x, new_y) == (self.player.x, self.player.y):
                    self.attack(entity, self.player)
                elif not self.is_blocked(new_x, new_y):
                    entity.x = new_x
                    entity.y = new_y
    
    def next_level(self):
        """Go to the next dungeon level"""
        self.level += 1
        self.message_log.append(f"{Color.MESSAGE}You descend deeper into the dungeon... Level {self.level}{Color.RESET}")
        
        # Keep player's stats and inventory
        player_stats = (self.player.hp, self.player.max_hp, self.player.attack, self.player.defense, self.player.xp)
        player_inventory = self.player.inventory.copy()
        
        # Generate new level
        self.init_game()
        
        # Restore player's stats and inventory
        self.player.hp, self.player.max_hp, self.player.attack, self.player.defense, self.player.xp = player_stats
        self.player.inventory = player_inventory
    
    def update_fov(self):
        """Update the player's field of view"""
        self.visible_tiles = set()
        
        # Add all tiles in a square around the player
        for x in range(self.player.x - FOV_RADIUS, self.player.x + FOV_RADIUS + 1):
            for y in range(self.player.y - FOV_RADIUS, self.player.y + FOV_RADIUS + 1):
                # Check if the tile is within the dungeon bounds
                if 0 <= x < DUNGEON_WIDTH and 0 <= y < DUNGEON_HEIGHT:
                    # Calculate the distance
                    distance = ((x - self.player.x) ** 2 + (y - self.player.y) ** 2) ** 0.5
                    
                    if distance <= FOV_RADIUS:
                        # Simple raycasting to check if the tile is visible
                        visible = True
                        
                        # Cast a ray from player to tile
                        if distance > 1.5:  # No need to check adjacent tiles
                            dx = x - self.player.x
                            dy = y - self.player.y
                            steps = max(abs(dx), abs(dy))
                            
                            if steps > 0:
                                dx /= steps
                                dy /= steps
                                
                                rx, ry = self.player.x, self.player.y
                                
                                for i in range(int(steps)):
                                    rx += dx
                                    ry += dy
                                    
                                    # Check if this point is a wall
                                    check_x, check_y = int(round(rx)), int(round(ry))
                                    if (check_x, check_y) in self.dungeon and self.dungeon[(check_x, check_y)] == WALL:
                                        visible = False
                                        break
                        
                        if visible:
                            self.visible_tiles.add((x, y))
                            self.explored_tiles.add((x, y))
    
    def render(self):
        """Render the game state to the terminal"""
        os.system('cls' if os.name == 'nt' else 'clear')
        
        # Update FOV
        self.update_fov()
        
        # Render dungeon
        for y in range(DUNGEON_HEIGHT):
            for x in range(DUNGEON_WIDTH):
                pos = (x, y)
                
                if pos in self.visible_tiles:
                    # Tile is visible
                    if pos in self.dungeon:
                        tile = self.dungeon[pos]
                        if tile == WALL:
                            print(f"{Color.WALL}{WALL}{Color.RESET}", end='')
                        elif tile == FLOOR:
                            print(f"{Color.FLOOR}{FLOOR}{Color.RESET}", end='')
                        elif tile == STAIRS:
                            print(f"{Color.STAIRS}{STAIRS}{Color.RESET}", end='')
                        else:
                            print(tile, end='')
                    else:
                        print(' ', end='')
                elif pos in self.explored_tiles:
                    # Tile has been seen before but is not currently visible
                    if pos in self.dungeon:
                        tile = self.dungeon[pos]
                        if tile == WALL:
                            print(f"\033[0;30m{WALL}{Color.RESET}", end='')  # Dark gray
                        elif tile == FLOOR:
                            print(f"\033[0;30m{FLOOR}{Color.RESET}", end='')  # Dark gray
                        elif tile == STAIRS:
                            print(f"\033[0;30m{STAIRS}{Color.RESET}", end='')  # Dark gray
                        else:
                            print(f"\033[0;30m{tile}{Color.RESET}", end='')
                    else:
                        print(' ', end='')
                else:
                    # Unexplored tile
                    print(FOG, end='')
            print()
        
        # Render entities
        for entity in self.entities:
            if (entity.x, entity.y) in self.visible_tiles:
                # Save cursor position
                print(f"\033[s\033[{entity.y + 1};{entity.x + 1}H{entity.color}{entity.char}{Color.RESET}", end='')
        
        # Restore cursor position and draw UI
        print("\033[u")
        
        # Draw player stats
        print(f"═" * DUNGEON_WIDTH)
        print(f"{self.player.name} | HP: {self.player.hp}/{self.player.max_hp} | ATK: {self.player.attack} | DEF: {self.player.defense} | XP: {self.player.xp} | Level: {self.level}")
        
        # Draw inventory
        if self.player.inventory:
            print(f"Inventory: {', '.join(self.player.inventory)}")
        else:
            print("Inventory: Empty")
        
        # Draw message log (last 3 messages)
        print(f"═" * DUNGEON_WIDTH)
        for message in self.message_log[-3:]:
            print(message)
        
        # Draw controls help
        print(f"═" * DUNGEON_WIDTH)
        print("Controls: Arrow keys to move, (q)uit")

def main():
    """Main game loop"""
    game = Game()
    game.init_game()
    
    while not game.game_over:
        game.render()
        
        # Get player input
        try:
            key = input("Enter command: ")
            
            if key.lower() == 'q':
                print("Thanks for playing!")
                break
            
            # Process player movement
            dx, dy = 0, 0
            if key.lower() == 'w':
                dy = -1
            elif key.lower() == 's':
                dy = 1
            elif key.lower() == 'a':
                dx = -1
            elif key.lower() == 'd':
                dx = 1
            elif key == '':  # Handle arrow keys
                key = key.strip()
                if key == '\x1b[A':  # Up arrow
                    dy = -1
                elif key == '\x1b[B':  # Down arrow
                    dy = 1
                elif key == '\x1b[D':  # Left arrow
                    dx = -1
                elif key == '\x1b[C':  # Right arrow
                    dx = 1
            
            if dx != 0 or dy != 0:
                game.move_player(dx, dy)
                game.process_enemy_turns()
            
        except KeyboardInterrupt:
            print("\nThanks for playing!")
            break
    
    if game.game_over:
        game.render()
        print("\nGame Over! Press any key to exit...")
        input()

if __name__ == "__main__":
    main()
