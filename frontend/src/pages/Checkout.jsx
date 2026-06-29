import React, { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { useCart } from '../contexts/CartContext';
import { pedidoService } from '../services/pedidoService';
import toast from 'react-hot-toast';
import { FaCreditCard, FaLock, FaCalendarAlt, FaUser } from 'react-icons/fa';

export const Checkout = () => {
  const { cart, loadCart } = useCart();
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);

  React.useEffect(() => {
    document.title = 'Finalizar Compra — TechStore';
  }, []);

  const handleFinishOrder = async (e) => {
    e.preventDefault();
    setLoading(true);
    try {
      const order = await pedidoService.criar();
      toast.success(`Pedido #${order.id} realizado com sucesso!`);
      await loadCart();
      navigate('/pedidos');
    } catch (err) {
      const msg = err.response?.data?.erro || 'Erro ao finalizar pedido';
      toast.error(msg);
    } finally {
      setLoading(false);
    }
  };

  if (!cart || cart.itens.length === 0) {
    return (
      <div className="container" style={{ textAlign: 'center', padding: '80px 24px' }}>
        <h2 style={{ marginBottom: '16px' }}>Nenhum item para checkout</h2>
        <Link to="/produtos" className="btn btn-primary">Ir para loja</Link>
      </div>
    );
  }

  return (
    <div className="container animate-fade-in">
      <h1 style={{ fontSize: '2rem', fontWeight: 800, marginBottom: '32px' }}>Finalizar Compra</h1>

      <div style={{ display: 'flex', gap: '32px', flexWrap: 'wrap' }}>
        {}
        <div style={{ flex: 2, minWidth: '320px' }}>
          <div className="glass" style={{ padding: '32px', borderRadius: 'var(--radius-lg)' }}>
            <h3 style={{ fontSize: '1.25rem', fontWeight: 700, marginBottom: '24px', display: 'flex', alignItems: 'center', gap: '10px' }}>
              <FaCreditCard style={{ color: 'var(--accent-primary)' }} /> Detalhes do Pagamento
            </h3>

            <form onSubmit={handleFinishOrder}>
              <div className="form-group">
                <label className="form-label" htmlFor="card-name">Nome no Cartão</label>
                <input id="card-name" type="text" className="form-input" placeholder="João da Silva" required />
              </div>

              <div className="form-group">
                <label className="form-label" htmlFor="card-num">Número do Cartão</label>
                <input id="card-num" type="text" className="form-input" placeholder="4000 1234 5678 9010" required />
              </div>

              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '20px' }}>
                <div className="form-group">
                  <label className="form-label" htmlFor="card-exp">Validade</label>
                  <input id="card-exp" type="text" className="form-input" placeholder="12/30" required />
                </div>
                <div className="form-group">
                  <label className="form-label" htmlFor="card-cvv">CVV</label>
                  <input id="card-cvv" type="text" className="form-input" placeholder="123" required />
                </div>
              </div>

              <div style={{ background: 'rgba(255,255,255,0.03)', padding: '16px', borderRadius: 'var(--radius-md)', display: 'flex', alignItems: 'center', gap: '12px', marginBottom: '24px', fontSize: '0.85rem', color: 'var(--text-secondary)' }}>
                <FaLock style={{ color: 'var(--success)' }} />
                <span>Transação segura e encriptada. Seus dados estão protegidos de ponta a ponta.</span>
              </div>

              <button type="submit" className="btn btn-primary btn-block" disabled={loading}>
                {loading ? 'Processando...' : `Confirmar Pagamento — R$ ${cart.total.toFixed(2)}`}
              </button>
            </form>
          </div>
        </div>

        {}
        <div style={{ flex: 1, minWidth: '300px' }}>
          <div className="glass" style={{ padding: '24px', borderRadius: 'var(--radius-lg)' }}>
            <h3 style={{ fontSize: '1.2rem', fontWeight: 700, marginBottom: '20px', borderBottom: '1px solid var(--border-color)', paddingBottom: '12px' }}>
              Itens do Pedido
            </h3>

            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px', marginBottom: '20px' }}>
              {cart.itens.map((item) => (
                <div key={item.id} style={{ display: 'flex', justifyContent: 'space-between', fontSize: '0.95rem' }}>
                  <span style={{ color: 'var(--text-secondary)' }}>
                    {item.quantidade}x {item.produto_nome}
                  </span>
                  <span style={{ fontWeight: 600 }}>R$ {item.subtotal.toFixed(2)}</span>
                </div>
              ))}
            </div>

            <div style={{ display: 'flex', justifyContent: 'space-between', paddingTop: '16px', borderTop: '1px solid var(--border-color)', fontSize: '1.2rem', fontWeight: 800 }}>
              <span>Total</span>
              <span style={{ color: 'var(--accent-secondary)' }}>R$ {cart.total.toFixed(2)}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
