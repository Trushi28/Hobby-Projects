# EcoScan Frontend

A React application for checking product sustainability and environmental impact. This frontend connects to the EcoScan backend API to retrieve and display sustainability data for consumer products.

## Overview

The EcoScan frontend provides a user-friendly interface for:
- Searching products by name
- Viewing eco-scores and sustainability metrics
- Analyzing product environmental impact
- Finding eco-friendly alternatives
- Viewing nutrition and ingredients information

## Technologies Used

- React.js
- Axios for API requests
- CSS for styling
- LocalStorage for search history persistence

## Features

### Search Functionality
- Product search by name
- Recent search history (stored locally)
- Quick re-search from history

### Product Information Display
- Eco-Score visualization with color coding (A-E rating system)
- Product image display
- Carbon footprint information
- Packaging details
- Sustainability analysis
- Nutrition information grid
- Ingredients list

### Additional Features
- Eco-friendly alternatives suggestions
- Eco-shopping tips toggle
- Loading state indicators
- Error handling
- Responsive design

## Component Structure

The application is built with a main `App` component that handles:
- State management for search queries
- API communication
- Data rendering
- User interactions

## State Management

The application uses React's `useState` and `useEffect` hooks to manage:
- Search queries
- Product data
- Loading states
- Error handling
- Search history
- UI toggles (e.g., eco tips)

## API Integration

The frontend connects to the backend API via Axios, making GET requests to:
- `http://localhost:5000/product?name={productName}` - Fetches product sustainability data

## Installation

1. Ensure you have Node.js installed (v14 or newer recommended)

2. Install dependencies:
   ```bash
   cd Frontend
   npm install
   ```

3. Start the development server:
   ```bash
   npm start
   ```

4. The application will open in your browser at `http://localhost:3000`

## Configuration

- Update the API endpoint in `searchProduct` function if your backend runs on a different port or host

## Future Enhancements

- User accounts for saving favorite products
- Barcode scanning capability
- Improved visualization of environmental metrics
- Product comparison feature
- Social sharing integration