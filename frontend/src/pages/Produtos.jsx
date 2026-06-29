import React, { useState, useEffect } from 'react';
import { useSearchParams } from 'react-router-dom';
import { produtoService } from '../services/produtoService';
import { categoriaService } from '../services/categoriaService';
import { useCart } from '../contexts/CartContext';
import { FaSearch, FaChevronLeft, FaChevronRight } from 'react-icons/fa';

export const Produtos = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [products, setProducts] = useState([]);
  const [categories, setCategories] = useState([]);
  const [loading, setLoading] = useState(true);
  const { addItem } = useCart();

  const searchName = searchParams.get('nome') || '';
  const selectedCategory = searchParams.get('categoria_id') || '';
  const currentPage = parseInt(searchParams.get('pagina') || '1', 10);

  const [searchInput, setSearchInput] = useState(searchName);
  const [paginationInfo, setPaginationInfo] = useState({ total_paginas: 1 });

  useEffect(() => {
    document.title = 'Buscar Produtos — TechStore';
    loadCategories();
  }, []);

  useEffect(() => {
    loadProducts();
  }, [searchParams]);

  const loadCategories = async () => {
    try {
      const data = await categoriaService.listar();
      setCategories(data);
    } catch (err) {
      console.error(err);
    }
  };

  const loadProducts = async () => {
    setLoading(true);
    try {
      const data = await produtoService.listar({
        nome: searchName,
        categoria_id: selectedCategory,
        pagina: currentPage,
        por_pagina: 8,
      });
      setProducts(data.dados);
      setPaginationInfo({
        total: data.total,
        pagina: data.pagina,
        por_pagina: data.por_pagina,
        total_paginas: data.total_paginas,
      });
    } catch (err) {
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleSearchSubmit = (e) => {
    e.preventDefault();
    searchParams.set('nome', searchInput);
    searchParams.set('pagina', '1');
    setSearchParams(searchParams);
  };

  const handleCategoryChange = (catId) => {
    if (catId) {
      searchParams.set('categoria_id', catId);
    } else {
      searchParams.delete('categoria_id');
    }
    searchParams.set('pagina', '1');
    setSearchParams(searchParams);
  };

  const handlePageChange = (newPage) => {
    searchParams.set('pagina', newPage.toString());
    setSearchParams(searchParams);
  };

  return (
    <div className="container animate-fade-in">
      <h1 style={{ fontSize: '2rem', fontWeight: 800, marginBottom: '24px' }}>Nossos Produtos</h1>

      {}
      <div className="glass" style={{ padding: '20px', borderRadius: 'var(--radius-lg)', marginBottom: '32px' }}>
        <form onSubmit={handleSearchSubmit} style={{ display: 'flex', gap: '16px', flexWrap: 'wrap' }}>
          <div style={{ flex: 1, minWidth: '260px', position: 'relative' }}>
            <input
              type="text"
              className="form-input"
              style={{ paddingLeft: '44px' }}
              placeholder="Pesquisar por nome do produto..."
              value={searchInput}
              onChange={(e) => setSearchInput(e.target.value)}
            />
            <FaSearch style={{ position: 'absolute', left: '16px', top: '50%', transform: 'translateY(-50%)', color: 'var(--text-muted)' }} />
          </div>

          <div style={{ minWidth: '200px' }}>
            <select
              className="form-input"
              style={{ background: 'var(--bg-primary)', cursor: 'pointer' }}
              value={selectedCategory}
              onChange={(e) => handleCategoryChange(e.target.value)}
            >
              <option value="">Todas as Categorias</option>
              {categories.map((cat) => (
                <option key={cat.id} value={cat.id}>{cat.nome}</option>
              ))}
            </select>
          </div>

          <button type="submit" className="btn btn-primary">
            Filtrar
          </button>
        </form>
      </div>

      {}
      {loading ? (
        <div style={{ display: 'flex', justifyContent: 'center', padding: '80px' }}>
          <div className="loading-spinner"></div>
        </div>
      ) : products.length === 0 ? (
        <div style={{ textAlign: 'center', padding: '60px 20px', color: 'var(--text-secondary)' }}>
          <p style={{ fontSize: '1.2rem', marginBottom: '10px' }}>Nenhum produto encontrado.</p>
          <p>Tente ajustar a sua busca ou trocar de categoria.</p>
        </div>
      ) : (
        <>
          <div className="products-grid">
            {products.map((prod) => (
              <div key={prod.id} className="product-card glass">
                <div className="product-img-wrapper">
                  <div className="product-img-placeholder">📦</div>
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

          {}
          {paginationInfo.total_paginas > 1 && (
            <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', gap: '16px', marginTop: '40px' }}>
              <button
                onClick={() => handlePageChange(currentPage - 1)}
                disabled={currentPage === 1}
                className="btn btn-secondary"
                style={{ padding: '8px 16px', display: 'flex', alignItems: 'center' }}
              >
                <FaChevronLeft style={{ marginRight: '6px' }} /> Anterior
              </button>

              <span style={{ fontWeight: 600, color: 'var(--text-secondary)' }}>
                Página {currentPage} de {paginationInfo.total_paginas}
              </span>

              <button
                onClick={() => handlePageChange(currentPage + 1)}
                disabled={currentPage === paginationInfo.total_paginas}
                className="btn btn-secondary"
                style={{ padding: '8px 16px', display: 'flex', alignItems: 'center' }}
              >
                Próxima <FaChevronRight style={{ marginLeft: '6px' }} />
              </button>
            </div>
          )}
        </>
      )}
    </div>
  );
};
