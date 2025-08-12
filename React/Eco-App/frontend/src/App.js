import React, { useState, useEffect } from 'react';
import axios from 'axios';
import './App.css';

const App = () => {
  const [query, setQuery] = useState('');
  const [product, setProduct] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [searchHistory, setSearchHistory] = useState([]);
  const [showTips, setShowTips] = useState(false);
  
  const [ecoScoreColors] = useState({
    'A': '#1E8F4E', // dark green
    'B': '#75B000', // light green
    'C': '#FFC600', // yellow
    'D': '#FF8B00', // orange
    'E': '#E63E11', // red
    'N/A': '#7F8C8D' // gray
  });
  
  // Load search history from localStorage on component mount
  useEffect(() => {
    const savedHistory = localStorage.getItem('searchHistory');
    if (savedHistory) {
      try {
        setSearchHistory(JSON.parse(savedHistory));
      } catch (e) {
        console.error("Failed to parse search history:", e);
      }
    }
  }, []);
  
  const searchProduct = async (productName = query) => {
    if (!productName) return;
    
    setLoading(true);
    setError(null);
    setProduct(null);
    
    try {
      // Make sure to use the correct API endpoint URL
      // Change this URL to your actual backend API URL in production
      const response = await axios.get(`http://localhost:5000/product?name=${encodeURIComponent(productName)}`);
      
      // Check if we received a valid response
      if (!response.data || response.status !== 200) {
        throw new Error('Invalid response from server');
      }
      
      // Process and structure the data
      const processedData = {
        productName: response.data.product.product_name || productName,
        image: response.data.product.image_url,
        ecoScore: response.data.ecoScore || 'N/A',
        carbonFootprint: response.data.carbonFootprint || 'Not available',
        analysis: response.data.analysis || 'No sustainability data available.',
        alternatives: response.data.alternatives || [],
        packaging: response.data.packaging || 'No packaging information',
        ingredients: response.data.ingredients || 'Ingredients not available',
        nutrition: response.data.nutrition || {
          score: 'N/A',
          energy: 'N/A',
          fat: 'N/A',
          sugars: 'N/A'
        }
      };
      
      // Set product data in state
      setProduct(processedData);
      
      // Update search history
      const updatedHistory = [productName, ...searchHistory.filter(item => item !== productName)].slice(0, 5);
      setSearchHistory(updatedHistory);
      localStorage.setItem('searchHistory', JSON.stringify(updatedHistory));
      
    } catch (err) {
      console.error("Search error:", err);
      setError(err.response?.data?.error || 'Failed to connect to the server. Please try again.');
    } finally {
      setLoading(false);
    }
  };
  
  const handleKeyPress = (e) => {
    if (e.key === 'Enter') {
      searchProduct();
    }
  };
  
  const renderEcoScore = (score) => {
    if (!score) score = 'N/A';
    const scoreUpper = score.toUpperCase();
    const color = ecoScoreColors[scoreUpper] || ecoScoreColors['N/A'];
    
    return (
      <div className="eco-score-container">
        <div className="eco-score-badge" style={{ backgroundColor: color }}>
          {scoreUpper}
        </div>
        <div className="eco-score-label">
          {getEcoScoreExplanation(scoreUpper)}
        </div>
      </div>
    );
  };
  
  const getEcoScoreExplanation = (score) => {
    const explanations = {
      'A': 'Excellent environmental impact',
      'B': 'Good environmental impact',
      'C': 'Moderate environmental impact',
      'D': 'High environmental impact',
      'E': 'Very high environmental impact',
      'N/A': 'Environmental impact unknown'
    };
    return explanations[score] || 'Environmental impact unknown';
  };
  
  return (
    <div className="container">
      <h1 className="title">EcoScan - Product Sustainability Checker</h1>
      
      <div className="search-section">
        <div className="search-bar">
          <input 
            type="text" 
            placeholder="Search for a product..." 
            value={query} 
            onChange={(e) => setQuery(e.target.value)}
            onKeyPress={handleKeyPress}
          />
          <button onClick={() => searchProduct()} disabled={loading}>
            {loading ? 'Searching...' : 'Search'}
          </button>
        </div>
        
        {searchHistory.length > 0 && (
          <div className="search-history">
            <p>Recent searches:</p>
            <div className="history-tags">
              {searchHistory.map((item, index) => (
                <button 
                  key={index} 
                  className="history-tag"
                  onClick={() => {
                    setQuery(item);
                    searchProduct(item);
                  }}
                >
                  {item}
                </button>
              ))}
            </div>
          </div>
        )}
        
        <button 
          className="tips-toggle" 
          onClick={() => setShowTips(!showTips)}
        >
          {showTips ? 'Hide Tips' : 'Show Eco Shopping Tips'}
        </button>
        
        {showTips && (
          <div className="eco-tips">
            <h3>Eco-Friendly Shopping Tips</h3>
            <ul>
              <li>Look for products with eco-scores A or B</li>
              <li>Choose items with minimal or recyclable packaging</li>
              <li>Consider local and seasonal products to reduce carbon footprint</li>
              <li>Plant-based alternatives often have lower environmental impact</li>
              <li>Check for certifications like Organic, Rainforest Alliance, or Fair Trade</li>
            </ul>
          </div>
        )}
      </div>
      
      {loading && (
        <div className="loading-spinner">
          <div className="spinner"></div>
          <p>Scanning product sustainability data...</p>
        </div>
      )}
      
      {error && <div className="error-message">{error}</div>}
      
      {product && (
        <div className="product-card">
          <div className="product-header">
            <h2>{product.productName}</h2>
            {renderEcoScore(product.ecoScore)}
          </div>
          
          <div className="product-content">
            <div className="product-image-container">
              {product.image ? (
                <img src={product.image} alt={product.productName} className="product-image" />
              ) : (
                <div className="no-image">
                  <span>Image not available</span>
                </div>
              )}
            </div>
            
            <div className="product-info">
              <div className="impact-meters">
                <div className="impact-meter">
                  <h3>Carbon Footprint</h3>
                  <div className="meter-value">{product.carbonFootprint}</div>
                </div>
                
                <div className="impact-meter">
                  <h3>Packaging</h3>
                  <div className="meter-value">{product.packaging}</div>
                </div>
              </div>
              
              <div className="analysis-section">
                <h3>Sustainability Analysis</h3>
                <p>{product.analysis}</p>
              </div>
              
              <div className="nutrition-section">
                <h3>Nutrition Information</h3>
                <div className="nutrition-grid">
                  <div className="nutrition-item">
                    <span className="nutrition-label">Nutri-Score:</span>
                    <span className="nutrition-value">
                      {product.nutrition.score ? product.nutrition.score.toUpperCase() : 'N/A'}
                    </span>
                  </div>
                  <div className="nutrition-item">
                    <span className="nutrition-label">Energy:</span>
                    <span className="nutrition-value">
                      {product.nutrition.energy !== 'N/A' ? 
                        `${product.nutrition.energy} kcal/100g` : 'N/A'}
                    </span>
                  </div>
                  <div className="nutrition-item">
                    <span className="nutrition-label">Fat:</span>
                    <span className="nutrition-value">
                      {product.nutrition.fat !== 'N/A' ? 
                        `${product.nutrition.fat}g/100g` : 'N/A'}
                    </span>
                  </div>
                  <div className="nutrition-item">
                    <span className="nutrition-label">Sugars:</span>
                    <span className="nutrition-value">
                      {product.nutrition.sugars !== 'N/A' ? 
                        `${product.nutrition.sugars}g/100g` : 'N/A'}
                    </span>
                  </div>
                </div>
              </div>
              
              <div className="ingredients-section">
                <h3>Ingredients</h3>
                <p className="ingredients-text">{product.ingredients}</p>
              </div>
            </div>
          </div>
          
          <div className="alternatives-section">
            <h3>Eco-friendly Alternatives</h3>
            {product.alternatives && product.alternatives.length > 0 ? (
              <div className="alternatives-grid">
                {product.alternatives.map((alt, index) => (
                  <div key={index} className="alternative-item">
                    <div className="alternative-header">
                      <span className="alternative-name">{alt.name}</span>
                      <span className="alternative-score" style={{
                        backgroundColor: ecoScoreColors[alt.eco_score?.toUpperCase()] || ecoScoreColors['N/A']
                      }}>
                        {alt.eco_score?.toUpperCase() || 'N/A'}
                      </span>
                    </div>
                    {alt.image_url && (
                      <img 
                        src={alt.image_url} 
                        alt={alt.name} 
                        className="alternative-image" 
                      />
                    )}
                    {alt.reason && (
                      <div className="alternative-reason">
                        <span>Why it's better:</span> {alt.reason}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            ) : (
              <p className="no-alternatives">No specific alternatives found for this product.</p>
            )}
          </div>
          
          <div className="eco-actions">
            <h3>Take Action</h3>
            <div className="action-buttons">
              <button className="action-button">
                Find Local Stores
              </button>
              <button className="action-button">
                Share Results
              </button>
              <button className="action-button">
                Compare Products
              </button>
            </div>
          </div>
        </div>
      )}
      
      <footer className="app-footer">
        <p>EcoScan uses data from Open Food Facts and environmental databases.</p>
        <p>© 2025 EcoScan - Making sustainable choices easier</p>
      </footer>
    </div>
  );
};

export default App;
