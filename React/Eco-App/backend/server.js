const express = require('express');
const axios = require('axios');
const cors = require('cors');
require('dotenv').config();
const { createClient } = require('@supabase/supabase-js');

const app = express();
app.use(express.json());
app.use(cors({ origin: '*' }));

const SUPABASE_URL = process.env.SUPABASE_URL;
const SUPABASE_KEY = process.env.SUPABASE_ANON_KEY;
const supabase = createClient(SUPABASE_URL, SUPABASE_KEY);

const generateAnalysis = (product) => {
  if (!product) return "No product data available.";
  
  const ecoScore = product.ecoscore_grade?.toUpperCase() || 'N/A';
  const ecoImpact = {
    'A': 'minimal',
    'B': 'low',
    'C': 'moderate',
    'D': 'high',
    'E': 'very high',
    'N/A': 'unknown'
  };
  
  const estimatedCarbonByCategory = {
    'en:plant-based-foods': '0.5 to 2.0',
    'en:dairy': '2.0 to 6.0',
    'en:meat': '8.0 to 30.0',
    'en:beverages': '0.2 to 1.0',
    'en:snacks': '1.0 to 3.0',
    'default': '1.0 to 5.0'
  };
  
  let carbonFootprint = 'Not available';
  if (product.carbon_footprint_per_100g) {
    carbonFootprint = `${product.carbon_footprint_per_100g} kg CO₂/100g`;
  } else if (product.categories_tags && product.categories_tags.length > 0) {
    for (const category of product.categories_tags) {
      if (estimatedCarbonByCategory[category]) {
        carbonFootprint = `Estimated ${estimatedCarbonByCategory[category]} kg CO₂/100g (based on category)`;
        break;
      }
    }
    if (carbonFootprint === 'Not available') {
      carbonFootprint = `Estimated ${estimatedCarbonByCategory.default} kg CO₂/100g (generalized estimate)`;
    }
  }
  
  let analysis = `This product has a ${ecoImpact[ecoScore]} environmental impact`;
  
  if (carbonFootprint !== 'Not available') {
    analysis += ` with ${carbonFootprint}.`;
  } else {
    analysis += '.';
  }
  
  if (product.packaging) {
    analysis += ` Packaging: ${product.packaging}.`;
  }
  
  if (product.ingredients_analysis_tags) {
    if (product.ingredients_analysis_tags.includes('en:vegan')) {
      analysis += ' This product is vegan, which generally has a lower environmental footprint.';
    } else if (product.ingredients_analysis_tags.includes('en:vegetarian')) {
      analysis += ' This product is vegetarian, which generally has a moderate environmental footprint.';
    }
  }
  
  if (product.labels_tags) {
    if (product.labels_tags.includes('en:organic')) {
      analysis += ' Grown using organic farming methods, which typically uses fewer synthetic pesticides.';
    }
    if (product.labels_tags.includes('en:fair-trade')) {
      analysis += ' Fair Trade certified, promoting sustainable livelihoods for producers.';
    }
    if (product.labels_tags.includes('en:local')) {
      analysis += ' Locally produced, which may reduce transportation emissions.';
    }
  }
  
  if (ecoScore === 'N/A') {
    analysis += ' Consider products with clear eco-labels for better environmental choices.';
  } else if (['D', 'E'].includes(ecoScore)) {
    analysis += ' Consider alternatives with better eco-scores for reduced environmental impact.';
  }
  
  return analysis;
};

const getAlternatives = async (product) => {
  try {
    if (supabase) {
      try {
        const { data: dbAlternatives } = await supabase
          .from('alternatives')
          .select('*')
          .eq('category', product.categories_tags?.[0] || 'unknown')
          .limit(5);
        
        if (dbAlternatives && dbAlternatives.length > 0) {
          return dbAlternatives;
        }
      } catch (error) {
        console.log("Supabase alternatives query failed:", error.message);
      }
    }
    
    if (product.categories_tags && product.categories_tags.length > 0) {
      const category = product.categories_tags[0];
      const response = await axios.get(
        `https://world.openfoodfacts.org/cgi/search.pl?action=process&tagtype_0=categories&tag_contains_0=contains&tag_0=${category}&sort_by=ecoscore_score&page_size=5&json=1`
      );
      
      if (response.data.products && response.data.products.length > 0) {
        const alternatives = response.data.products
          .filter(alt => 
            alt.product_name &&
            alt._id !== product._id
          )
          .map(alt => ({
            name: alt.product_name,
            eco_score: alt.ecoscore_grade || 'N/A',
            image_url: alt.image_url,
            reason: determineEcoAdvantage(alt, product)
          }));
        
        return alternatives.slice(0, 3); 
      }
    }
    
    const genericAlternatives = {
      'en:beverages': [
        { name: 'Filtered Tap Water', eco_score: 'A', reason: 'Zero packaging waste, minimal carbon footprint' },
        { name: 'Locally Sourced Juice', eco_score: 'B', reason: 'Reduced transportation emissions' }
      ],
      'en:snacks': [
        { name: 'Local Fresh Fruit', eco_score: 'A', reason: 'Minimal processing, reduced transportation' },
        { name: 'Organic Nuts', eco_score: 'B', reason: 'Sustainable farming practices' }
      ],
      'en:dairy': [
        { name: 'Plant-based Milk Alternative', eco_score: 'B', reason: 'Lower resource consumption' },
        { name: 'Local Organic Dairy', eco_score: 'C', reason: 'Reduced transportation emissions' }
      ],
      'default': [
        { name: 'Locally Sourced Alternative', eco_score: 'B', reason: 'Reduced transportation emissions' },
        { name: 'Package-free Option', eco_score: 'A', reason: 'Eliminates packaging waste' }
      ]
    };
    
    for (const category of product.categories_tags || []) {
      if (genericAlternatives[category]) {
        return genericAlternatives[category];
      }
    }
    
    return genericAlternatives.default;
  } catch (error) {
    console.error("Error fetching alternatives:", error);
    return [
      { name: 'Local/Seasonal Products', eco_score: 'A', reason: 'Lower transportation emissions' },
      { name: 'Package-free Options', eco_score: 'A', reason: 'Reduced packaging waste' },
      { name: 'Plant-based Alternatives', eco_score: 'B', reason: 'Lower resource consumption' }
    ];
  }
};

function determineEcoAdvantage(alternative, original) {
  let reasons = [];
  
  if (alternative.ecoscore_grade && original.ecoscore_grade) {
    if (alternative.ecoscore_grade < original.ecoscore_grade) {
      reasons.push('Better eco-score rating');
    }
  }
  
  if (alternative.packaging_tags && alternative.packaging_tags.includes('en:recyclable-packaging')) {
    reasons.push('Recyclable packaging');
  }
  
  if (alternative.labels_tags) {
    if (alternative.labels_tags.includes('en:organic')) {
      reasons.push('Organic production');
    }
    if (alternative.labels_tags.includes('en:fair-trade')) {
      reasons.push('Fair trade certified');
    }
  }
  
  if (alternative.carbon_footprint_per_100g && original.carbon_footprint_per_100g) {
    if (alternative.carbon_footprint_per_100g < original.carbon_footprint_per_100g) {
      reasons.push('Lower carbon footprint');
    }
  }
  
  if (reasons.length === 0) {
    reasons.push('Alternative option');
  }
  
  return reasons.join(', ');
}

app.get('/product', async (req, res) => {
  const { name } = req.query;
  if (!name) {
    return res.status(400).json({ error: 'Please provide a product name' });
  }
  
  try {
    console.log(`Searching for product: ${name}`);
    const response = await axios.get(
      `https://world.openfoodfacts.org/cgi/search.pl?search_terms=${encodeURIComponent(name)}&json=1&page_size=1`
    );
    
    if (!response.data.products || response.data.products.length === 0) {
      return res.status(404).json({ error: 'Product not found. Try another name.' });
    }
    
    const product = response.data.products[0];
    console.log(`Found product: ${product.product_name || name}`);
    
    const analysis = generateAnalysis(product);
    const carbonFootprint = product.carbon_footprint_per_100g 
      ? `${product.carbon_footprint_per_100g} kg CO₂/100g` 
      : estimateFootprint(product);
    
    let ecoScore = product.ecoscore_grade || estimateEcoScore(product);
    
    // Store search record in Supabase
    if (supabase && SUPABASE_URL && SUPABASE_KEY) {
      try {
        await supabase.from('searched_products').upsert({
          product_id: product._id || `${name}-${Date.now()}`,
          product_name: product.product_name || name,
          search_count: 1,
          last_searched: new Date(),
          ecoscore: ecoScore
        }, { onConflict: 'product_id', ignoreDuplicates: false });
      } catch (dbError) {
        console.error("Supabase error (non-critical):", dbError.message);
      }
    }
    
    const alternatives = await getAlternatives(product);
    
    const productData = {
      product: {
        ...product,
        product_name: product.product_name || name
      },
      ecoScore: ecoScore,
      carbonFootprint: carbonFootprint,
      analysis: analysis,
      packaging: product.packaging || 'Information not available',
      ingredients: product.ingredients_text || 'Information not available',
      nutrition: {
        score: product.nutrition_grade_fr || 'N/A',
        energy: product.nutriments?.energy_100g || 'N/A',
        fat: product.nutriments?.fat_100g || 'N/A',
        sugars: product.nutriments?.sugars_100g || 'N/A'
      },
      alternatives: alternatives
    };
    
    res.json(productData);
  } catch (error) {
    console.error("API Error:", error.message);
    if (error.response) {
      console.error("Response status:", error.response.status);
    }
    res.status(500).json({ error: 'Error fetching product data. Please try again later.' });
  }
});

function estimateFootprint(product) {
  const categoryEstimates = {
    'en:plant-based-foods': '0.5-2.0 kg CO₂/100g (estimated)',
    'en:dairy': '2.0-6.0 kg CO₂/100g (estimated)',
    'en:meat': '8.0-30.0 kg CO₂/100g (estimated)',
    'en:fish': '3.0-7.0 kg CO₂/100g (estimated)',
    'en:beverages': '0.2-1.0 kg CO₂/100g (estimated)',
    'en:snacks': '1.0-3.0 kg CO₂/100g (estimated)'
  };
  
  if (product.categories_tags) {
    for (const category of product.categories_tags) {
      if (categoryEstimates[category]) {
        return categoryEstimates[category];
      }
    }
  }
  
  return 'Not available (data missing)';
}

function estimateEcoScore(product) {
  if (product.nutrition_grade_fr) {
    const nutritionToEco = {
      'a': 'C', 'b': 'C', 'c': 'D', 'd': 'D', 'e': 'D'
    };
    return nutritionToEco[product.nutrition_grade_fr.toLowerCase()] || 'D';
  }
  
  if (product.labels_tags && product.labels_tags.includes('en:organic')) {
    return 'B';
  }
  
  return 'N/A';
}

app.get('/health', (req, res) => {
  res.json({ status: 'healthy', message: 'EcoScan API is running' });
});

const PORT = process.env.PORT || 5000;
app.listen(PORT, () => console.log(`Server running on port ${PORT}`));
