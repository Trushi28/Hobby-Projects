# EcoScan - Product Sustainability Checker

A full-stack application that helps users make environmentally conscious purchasing decisions by providing sustainability information and eco-friendly alternatives for consumer products.

## Project Structure

```
Main Folder
├── Frontend/ (React application)
└── Backend/ (Node.js Express server)
```

## Technologies Used

### Frontend
- React.js
- Axios for API requests
- CSS for styling
- LocalStorage for search history persistence

### Backend
- Node.js
- Express.js
- Axios for external API requests
- Supabase for data storage
- Open Food Facts API integration

## Features

- **Product Search**: Look up products by name
- **Eco-Score Rating**: Visual A-E rating system for environmental impact
- **Sustainability Analysis**: Detailed breakdown of environmental factors
- **Carbon Footprint**: Actual or estimated carbon emissions data
- **Eco-friendly Alternatives**: Suggestions for more sustainable products
- **Nutrition & Ingredients**: Comprehensive product information
- **Search History**: Recent searches saved for quick access
- **Shopping Tips**: Eco-friendly shopping guidance

## Getting Started

### Prerequisites
- Node.js (v14.0.0 or later recommended)
- npm (v6.0.0 or later recommended)
- Supabase account (for backend database)

### Installation

1. Clone the repository
   ```bash
   git clone https://github.com/Trushi28/Eco-App.git
   cd Eco-App
   ```

2. Set up the frontend
   ```bash
   cd Frontend
   npm install
   ```

3. Set up the backend
   ```bash
   cd ../Backend
   npm install
   ```

4. Create a `.env` file in the Backend directory with your Supabase credentials:
   ```
   PORT=5000
   SUPABASE_URL=your_supabase_url
   SUPABASE_ANON_KEY=your_supabase_key
   ```

## Running the Application

### Running the Backend
```bash
cd Backend
npm start
```
The backend server will run on `http://localhost:5000`

### Running the Frontend
```bash
cd Frontend
npm start
```
The frontend will be available at `http://localhost:3000`

## API Documentation

### GET /product?name={productName}
Returns comprehensive product sustainability data including:
- Product details (name, image, etc.)
- Eco-Score grade
- Carbon footprint
- Sustainability analysis
- Eco-friendly alternatives
- Nutrition information
- Packaging details

### GET /health
Simple health check endpoint to verify API status

## Data Sources

EcoScan uses data from:
- Open Food Facts database
- Environmental impact estimations based on product categories
- Curated sustainable alternatives from Supabase database

## Future Enhancements

- Barcode scanning functionality
- User accounts for saved products and preferences
- Product comparison feature
- Social sharing capabilities
- Mobile application version
- Machine learning for improved impact estimates

## Contributing

1. Fork the project
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is currently unlicensed. All rights reserved.

## Contact

Your Name - [trushibethu@.com](mailto:trushibethu@gmail.com)

Project Link: [https://github.com/Trushi28/Eco-App](https://github.com/Trushi28/Eco-App)

## Acknowledgments

- Open Food Facts for their comprehensive food product database
- Supabase for providing backend database services
- All contributors who have helped shape and improve this project
