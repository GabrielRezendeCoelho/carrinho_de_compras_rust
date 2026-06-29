import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { categoriaService } from '../services/categoriaService';
import { FaTag } from 'react-icons/fa';

export const Categorias = () => {
  const [categories, setCategories] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    document.title = 'Categorias — TechStore';
    const loadCategories = async () => {
      try {
        const data = await categoriaService.listar();
        setCategories(data);
      } catch (err) {
        console.error(err);
      } finally {
        setLoading(false);
      }
    };
    loadCategories();
  }, []);

  return (
    <div className="container animate-fade-in">
      <h1 style={{ fontSize: '2rem', fontWeight: 800, marginBottom: '24px' }}>Categorias</h1>

      {loading ? (
        <div style={{ display: 'flex', justifyContent: 'center', padding: '80px' }}>
          <div className="loading-spinner"></div>
        </div>
      ) : (
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(260px, 1fr))', gap: '24px' }}>
          {categories.map((cat) => (
            <Link
              key={cat.id}
              to={`/produtos?categoria_id=${cat.id}`}
              className="glass"
              style={{
                padding: '30px 24px',
                borderRadius: 'var(--radius-lg)',
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                gap: '16px',
                transition: 'all var(--transition-normal)'
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.transform = 'translateY(-6px)';
                e.currentTarget.style.borderColor = 'var(--accent-primary)';
                e.currentTarget.style.boxShadow = 'var(--shadow-lg)';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.transform = 'translateY(0)';
                e.currentTarget.style.borderColor = 'var(--border-color)';
                e.currentTarget.style.boxShadow = 'none';
              }}
            >
              <div style={{ background: 'rgba(99, 102, 241, 0.1)', width: '60px', height: '60px', borderRadius: 'var(--radius-md)', display: 'flex', alignItems: 'center', justify: 'center', color: 'var(--accent-primary)', fontSize: '1.5rem' }}>
                <FaTag />
              </div>
              <h3 style={{ fontSize: '1.2rem', fontWeight: 700 }}>{cat.nome}</h3>
              <p style={{ fontSize: '0.9rem', color: 'var(--text-secondary)' }}>Ver todos os produtos</p>
            </Link>
          ))}
        </div>
      )}
    </div>
  );
};
