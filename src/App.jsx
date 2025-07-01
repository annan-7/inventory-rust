import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

function App() {
  const [products, setProducts] = useState([]);
  const [categories, setCategories] = useState([]);
  const [selectedCategory, setSelectedCategory] = useState('all');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  
  // Form state
  const [formData, setFormData] = useState({
    name: '',
    category: '',
    quantity: '',
    price: ''
  });
  const [editingId, setEditingId] = useState(null);
  const [showForm, setShowForm] = useState(false);

  // Load products and categories on component mount
  useEffect(() => {
    loadData();
  }, []);

  // Load products when category filter changes
  useEffect(() => {
    if (selectedCategory === 'all') {
      loadProducts();
    } else {
      loadProductsByCategory(selectedCategory);
    }
  }, [selectedCategory]);

  const loadData = async () => {
    try {
      setLoading(true);
      await Promise.all([loadProducts(), loadCategories()]);
    } catch (err) {
      setError('Failed to load data: ' + err);
    } finally {
      setLoading(false);
    }
  };

  const loadProducts = async () => {
    try {
      const result = await invoke('get_products');
      setProducts(result);
    } catch (err) {
      setError('Failed to load products: ' + err);
    }
  };

  const loadProductsByCategory = async (category) => {
    try {
      const result = await invoke('get_products_by_category', { category });
      setProducts(result);
    } catch (err) {
      setError('Failed to load products by category: ' + err);
    }
  };

  const loadCategories = async () => {
    try {
      const result = await invoke('get_categories');
      setCategories(result);
    } catch (err) {
      setError('Failed to load categories: ' + err);
    }
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      if (editingId) {
        await invoke('update_product', {
          id: editingId,
          name: formData.name,
          category: formData.category,
          quantity: parseInt(formData.quantity),
          price: parseFloat(formData.price)
        });
      } else {
        await invoke('create_product', {
          name: formData.name,
          category: formData.category,
          quantity: parseInt(formData.quantity),
          price: parseFloat(formData.price)
        });
      }
      
      resetForm();
      await loadData();
    } catch (err) {
      setError('Failed to save product: ' + err);
    }
  };

  const handleDelete = async (id) => {
    if (window.confirm('Are you sure you want to delete this product?')) {
      try {
        await invoke('delete_product', { id });
        await loadData();
      } catch (err) {
        setError('Failed to delete product: ' + err);
      }
    }
  };

  const handleEdit = (product) => {
    setFormData({
      name: product.name,
      category: product.category,
      quantity: product.quantity.toString(),
      price: product.price.toString()
    });
    setEditingId(product.id);
    setShowForm(true);
  };

  const resetForm = () => {
    setFormData({ name: '', category: '', quantity: '', price: '' });
    setEditingId(null);
    setShowForm(false);
  };

  const formatDate = (dateString) => {
    return new Date(dateString).toLocaleDateString();
  };

  if (loading) {
    return <div className="loading">Loading...</div>;
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>📦 Inventory Management System</h1>
        <button 
          className="btn btn-primary"
          onClick={() => setShowForm(true)}
        >
          + Add Product
        </button>
      </header>

      {error && (
        <div className="error-message">
          {error}
          <button onClick={() => setError('')}>×</button>
        </div>
      )}

      <div className="main-content">
        <div className="sidebar">
          <h3>Categories</h3>
          <div className="category-list">
            <button
              className={`category-item ${selectedCategory === 'all' ? 'active' : ''}`}
              onClick={() => setSelectedCategory('all')}
            >
              All Products ({products.length})
            </button>
            {categories.map((category) => (
              <button
                key={category.name}
                className={`category-item ${selectedCategory === category.name ? 'active' : ''}`}
                onClick={() => setSelectedCategory(category.name)}
              >
                {category.name} ({category.count})
              </button>
            ))}
          </div>
        </div>

        <div className="content">
          {showForm && (
            <div className="form-overlay">
              <div className="form-container">
                <h2>{editingId ? 'Edit Product' : 'Add New Product'}</h2>
                <form onSubmit={handleSubmit}>
                  <div className="form-group">
                    <label>Name:</label>
                    <input
                      type="text"
                      value={formData.name}
                      onChange={(e) => setFormData({...formData, name: e.target.value})}
                      required
                    />
                  </div>
                  <div className="form-group">
                    <label>Category:</label>
                    <input
                      type="text"
                      value={formData.category}
                      onChange={(e) => setFormData({...formData, category: e.target.value})}
                      required
                    />
                  </div>
                  <div className="form-group">
                    <label>Quantity:</label>
                    <input
                      type="number"
                      value={formData.quantity}
                      onChange={(e) => setFormData({...formData, quantity: e.target.value})}
                      required
                      min="0"
                    />
                  </div>
                  <div className="form-group">
                    <label>Price:</label>
                    <input
                      type="number"
                      step="0.01"
                      value={formData.price}
                      onChange={(e) => setFormData({...formData, price: e.target.value})}
                      required
                      min="0"
                    />
                  </div>
                  <div className="form-actions">
                    <button type="submit" className="btn btn-primary">
                      {editingId ? 'Update' : 'Create'}
                    </button>
                    <button type="button" className="btn btn-secondary" onClick={resetForm}>
                      Cancel
                    </button>
                  </div>
                </form>
              </div>
            </div>
          )}

          <div className="products-grid">
            {products.map((product) => (
              <div key={product.id} className="product-card">
                <div className="product-header">
                  <h3>{product.name}</h3>
                  <span className="category-badge">{product.category}</span>
                </div>
                <div className="product-details">
                  <div className="detail-item">
                    <span className="label">Quantity:</span>
                    <span className={`value ${product.quantity < 10 ? 'low-stock' : ''}`}>
                      {product.quantity}
                    </span>
                  </div>
                  <div className="detail-item">
                    <span className="label">Price:</span>
                    <span className="value">${product.price.toFixed(2)}</span>
                  </div>
                  <div className="detail-item">
                    <span className="label">Total Value:</span>
                    <span className="value">${(product.quantity * product.price).toFixed(2)}</span>
                  </div>
                  <div className="detail-item">
                    <span className="label">Created:</span>
                    <span className="value">{formatDate(product.created_at)}</span>
                  </div>
                </div>
                <div className="product-actions">
                  <button 
                    className="btn btn-small btn-secondary"
                    onClick={() => handleEdit(product)}
                  >
                    Edit
                  </button>
                  <button 
                    className="btn btn-small btn-danger"
                    onClick={() => handleDelete(product.id)}
                  >
                    Delete
                  </button>
                </div>
              </div>
            ))}
          </div>

          {products.length === 0 && (
            <div className="empty-state">
              <p>No products found. Add your first product to get started!</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App; 