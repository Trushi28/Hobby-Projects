# EcoScan Backend API

A Node.js Express server that powers the EcoScan product sustainability application. This API retrieves product data, analyzes environmental impact, and suggests eco-friendly alternatives.

## Overview

The EcoScan backend API connects to the Open Food Facts database to retrieve product information, then enriches this data with sustainability metrics, carbon footprint estimates, and eco-friendly alternatives.

## Technologies Used

- Node.js
- Express.js
- Axios for external API requests
- Supabase for data storage
- Dotenv for environment variable management
- CORS for cross-origin request handling

## API Endpoints

### Product Information
- **GET** `/product?name={productName}`
  - Retrieves comprehensive sustainability data for a product
  - Returns eco-score, carbon footprint, analysis, and alternatives

### Health Check
- **GET** `/health`
  - Simple health check endpoint to verify API status

## Data Processing

The API performs several key functions:

1. **Product Lookup**: Searches Open Food Facts database for product information
2. **Sustainability Analysis**: Generates environmental impact analysis based on product data
3. **Carbon Footprint Estimation**: Uses actual data when available or estimates based on product category
4. **Alternative Suggestions**: Provides eco-friendly product alternatives
5. **Data Persistence**: Stores search history and product data in Supabase

## Key Features

### Environmental Analysis
The `generateAnalysis` function creates a textual analysis of a product's environmental impact based on:
- Eco-Score grade (A-E)
- Carbon footprint data
- Packaging information
- Ingredient analysis
- Certifications and eco-labels

### Alternative Products
The `getAlternatives` function suggests more sustainable product alternatives by:
- Querying the Supabase database for pre-defined alternatives
- Searching Open Food Facts for better-rated products in the same category
- Providing generic sustainable alternatives when specific data is unavailable

### Carbon Footprint Estimation
When precise carbon footprint data is unavailable, the API estimates emissions based on product category using established ranges.

## Database Integration

The API connects to a Supabase database to:
- Store search history and frequently requested products
- Retrieve curated eco-friendly alternatives
- Track product popularity and search trends

## Installation

1. Ensure you have Node.js installed (v14 or newer recommended)

2. Install dependencies:
   ```bash
   cd Backend
   npm install
   ```

3. Create a `.env` file with the following variables:
   ```
   PORT=5000
   SUPABASE_URL=your_supabase_url
   SUPABASE_ANON_KEY=your_supabase_key
   ```

4. Start the server:
   ```bash
   npm start
   ```

5. The API will be available at `http://localhost:5000`

## Dependencies

- express: Web server framework
- axios: HTTP client for external API requests
- cors: Cross-origin resource sharing middleware
- dotenv: Environment variable management
- @supabase/supabase-js: Supabase client library

## Future Enhancements

- User authentication for personalized recommendations
- Machine learning for more accurate carbon footprint estimations
- Integration with additional sustainability databases
- Caching layer for improved performance
- Advanced filtering and sorting options