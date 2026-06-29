import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { produtoService } from '../services/produtoService';
import { categoriaService } from '../services/categoriaService';
import { useCart } from '../contexts/CartContext';
import { FaArrowRight, FaTags, FaBoxOpen } from 'react-icons/fa';

export const Home = () => {
  const [featuredProducts, setFeaturedProducts] = useState([]);
  const [categories, setCategories] = useState([]);
  const [loading, setLoading] = useState(true);
  const { addItem } = useCart();

  useEffect(() => {
    document.title = 'TechStore — Melhores Produtos Tecnológicos';
    const fetchData = async () => {
      try {
        const prodData = await produtoService.listar({ por_pagina: 4 });
        setFeaturedProducts(prodData.dados);

        const catData = await categoriaService.listar();
        setCategories(catData.slice(0, 4));
      } catch (err) {
        console.error("Erro ao carregar dados da Home:", err);
      } finally {
        setLoading(false);
      }
    };
    fetchData();
  }, []);

  return (
    <div className="animate-fade-in">
      {}
      <section className="hero-section glass container" style={{ padding: '60px 40px', borderRadius: 'var(--radius-lg)', marginBottom: '40px', background: 'radial-gradient(circle at top right, rgba(99, 102, 241, 0.15) 0%, rgba(22, 31, 48, 0.7) 100%)' }}>
        <div style={{ maxWidth: '600px' }}>
          <h1 style={{ fontSize: '3rem', fontWeight: 800, lineHeight: 1.2, marginBottom: '20px' }}>
            As Melhores Ofertas. <br/>
            <span style={{ background: 'var(--accent-gradient)', WebkitBackgroundClip: 'text', WebkitTextFillColor: 'transparent' }}>
              Com os Menores Preços.
            </span>
          </h1>
          <p style={{ color: 'var(--text-secondary)', fontSize: '1.1rem', marginBottom: '30px' }}>
            Explore os melhores e mais modernos dispositivos eletrônicos, vestuário premium e muito mais, tudo integrado num sistema full stack de altíssima performance.
          </p>
          <div style={{ display: 'flex', gap: '16px' }}>
            <Link to="/produtos" className="btn btn-primary">
              Ver Produtos <FaArrowRight style={{ marginLeft: '8px' }} />
            </Link>
            <Link to="/categorias" className="btn btn-secondary">
              Categorias
            </Link>
          </div>
        </div>
      </section>

      {}
      <section className="container" style={{ marginBottom: '50px' }}>
        <h2 style={{ fontSize: '1.8rem', fontWeight: 700, marginBottom: '24px', display: 'flex', alignItems: 'center', gap: '10px' }}>
          <FaTags style={{ color: 'var(--accent-primary)' }} /> Compre por Categoria
        </h2>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(220px, 1fr))', gap: '20px' }}>
          {categories.map((cat) => (
            <Link
              key={cat.id}
              to={`/produtos?categoria_id=${cat.id}`}
              className="glass"
              style={{
                padding: '24px',
                borderRadius: 'var(--radius-md)',
                textAlign: 'center',
                fontWeight: 600,
                display: 'block',
                transition: 'all var(--transition-fast)'
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.transform = 'translateY(-4px)';
                e.currentTarget.style.borderColor = 'var(--accent-primary)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.transform = 'translateY(0)';
                e.currentTarget.style.borderColor = 'var(--border-color)';
              }}
            >
              {cat.nome}
            </Link>
          ))}
        </div>
      </section>

      {}
      <section className="container">
        <h2 style={{ fontSize: '1.8rem', fontWeight: 700, marginBottom: '24px', display: 'flex', alignItems: 'center', gap: '10px' }}>
          <FaBoxOpen style={{ color: 'var(--accent-secondary)' }} /> Produtos em Destaque
        </h2>

        {loading ? (
          <div style={{ display: 'flex', justifyContent: 'center', padding: '40px' }}>
            <div className="loading-spinner"></div>
          </div>
        ) : (
          <div className="products-grid">
            {featuredProducts.map((prod) => (
              <div key={prod.id} className="product-card glass">
                <div className="product-img-wrapper">
                  <div className="product-img-placeholder">💻</div>
                  <span className="product-category-tag">{prod.categoria_nome}</span>
                </div>
                <div className="product-info">
                  <h3 className="product-name">{prod.nome}</h3>
                  <p className="product-desc">{prod.descricao}</p>
                  <div className="product-footer">
                    <span className="product-price">{prod.preco.toFixed(2)}</span>
                    <button
                      onClick={() => addItem(prod.id, 1)}
                      className="add-cart-icon-btn"
                      title="Adicionar ao carrinho"
                    >
                      +
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </section>
    </div>
  );
};
