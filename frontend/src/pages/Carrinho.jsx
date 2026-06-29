import React from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useCart } from '../contexts/CartContext';
import { FaTrash, FaShoppingCart, FaArrowLeft, FaCreditCard } from 'react-icons/fa';

export const Carrinho = () => {
  const { cart, loading, updateQuantity, removeItem, clearCart } = useCart();
  const navigate = useNavigate();

  React.useEffect(() => {
    document.title = 'Seu Carrinho — TechStore';
  }, []);

  const handleCheckoutClick = () => {
    navigate('/checkout');
  };

  if (loading && !cart) {
    return (
      <div style={{ display: 'flex', justifyContent: 'center', padding: '80px' }}>
        <div className="loading-spinner"></div>
      </div>
    );
  }

  if (!cart || cart.itens.length === 0) {
    return (
      <div className="container animate-fade-in" style={{ textAlign: 'center', padding: '80px 24px' }}>
        <div style={{ fontSize: '4rem', color: 'var(--text-muted)', marginBottom: '24px' }}>
          <FaShoppingCart />
        </div>
        <h1 style={{ fontSize: '1.8rem', marginBottom: '12px' }}>Seu carrinho está vazio</h1>
        <p style={{ color: 'var(--text-secondary)', marginBottom: '32px' }}>Adicione produtos ao carrinho antes de prosseguir.</p>
        <Link to="/produtos" className="btn btn-primary">
          <FaArrowLeft style={{ marginRight: '8px' }} /> Continuar Comprando
        </Link>
      </div>
    );
  }

  return (
    <div className="container animate-fade-in">
      <h1 style={{ fontSize: '2rem', fontWeight: 800, marginBottom: '32px' }}>Seu Carrinho</h1>

      <div style={{ display: 'flex', gap: '32px', flexWrap: 'wrap', alignItems: 'flex-start' }}>
        {}
        <div style={{ flex: 2, minWidth: '320px' }}>
          <div className="glass" style={{ borderRadius: 'var(--radius-lg)', padding: '24px', display: 'flex', flexDirection: 'column', gap: '20px' }}>
            {cart.itens.map((item) => (
              <div
                key={item.id}
                style={{
                  display: 'flex',
                  justifyContent: 'space-between',
                  alignItems: 'center',
                  paddingBottom: '20px',
                  borderBottom: '1px solid var(--border-color)',
                  flexWrap: 'wrap',
                  gap: '16px'
                }}
              >
                {}
                <div style={{ flex: 1, minWidth: '180px' }}>
                  <h4 style={{ fontSize: '1.1rem', fontWeight: 700, marginBottom: '4px' }}>{item.produto_nome}</h4>
                  <p style={{ fontSize: '0.9rem', color: 'var(--text-secondary)' }}>Preço unitário: R$ {item.preco.toFixed(2)}</p>
                </div>

                {}
                <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                  <button
                    onClick={() => updateQuantity(item.id, item.quantidade - 1)}
                    disabled={item.quantidade <= 1}
                    className="btn btn-secondary"
                    style={{ padding: '6px 12px', minWidth: '36px' }}
                  >
                    -
                  </button>
                  <span style={{ fontWeight: 600, fontSize: '1.1rem', width: '20px', textAlign: 'center' }}>
                    {item.quantidade}
                  </span>
                  <button
                    onClick={() => updateQuantity(item.id, item.quantidade + 1)}
                    className="btn btn-secondary"
                    style={{ padding: '6px 12px', minWidth: '36px' }}
                  >
                    +
                  </button>
                </div>

                {}
                <div style={{ minWidth: '100px', textAlign: 'right' }}>
                  <span style={{ fontWeight: 700, fontSize: '1.15rem' }}>R$ {item.subtotal.toFixed(2)}</span>
                </div>

                {}
                <button
                  onClick={() => removeItem(item.id)}
                  style={{ cursor: 'pointer', color: 'var(--danger)', background: 'none', border: 'none', padding: '8px' }}
                  title="Remover produto"
                >
                  <FaTrash />
                </button>
              </div>
            ))}

            <div style={{ display: 'flex', justifyContent: 'space-between', marginTop: '12px' }}>
              <Link to="/produtos" className="btn btn-secondary" style={{ padding: '10px 20px' }}>
                <FaArrowLeft style={{ marginRight: '8px' }} /> Continuar Comprando
              </Link>
              <button onClick={clearCart} className="btn btn-secondary" style={{ padding: '10px 20px', color: 'var(--danger)', borderColor: 'rgba(239, 68, 68, 0.2)' }}>
                <FaTrash style={{ marginRight: '8px' }} /> Limpar Carrinho
              </button>
            </div>
          </div>
        </div>

        {}
        <div style={{ flex: 1, minWidth: '300px' }}>
          <div className="glass" style={{ padding: '24px', borderRadius: 'var(--radius-lg)' }}>
            <h3 style={{ fontSize: '1.3rem', fontWeight: 700, marginBottom: '20px', borderBottom: '1px solid var(--border-color)', paddingBottom: '12px' }}>
              Resumo do Pedido
            </h3>

            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '16px', color: 'var(--text-secondary)' }}>
              <span>Subtotal</span>
              <span>R$ {cart.total.toFixed(2)}</span>
            </div>
            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '24px', borderBottom: '1px solid var(--border-color)', paddingBottom: '16px', fontSize: '1.25rem', fontWeight: 800 }}>
              <span>Total</span>
              <span style={{ color: 'var(--accent-secondary)' }}>R$ {cart.total.toFixed(2)}</span>
            </div>

            <button
              onClick={handleCheckoutClick}
              className="btn btn-primary btn-block"
              style={{ display: 'flex', gap: '8px', justifyContent: 'center' }}
            >
              <FaCreditCard /> Ir para o Checkout
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
