import React, { useState, useEffect } from 'react';
import { pedidoService } from '../services/pedidoService';
import toast from 'react-hot-toast';
import { FaFileInvoiceDollar, FaCalendarAlt } from 'react-icons/fa';

export const Pedidos = () => {
  const [orders, setOrders] = useState([]);
  const [loading, setLoading] = useState(true);
  const [selectedOrder, setSelectedOrder] = useState(null);

  useEffect(() => {
    document.title = 'Meus Pedidos — TechStore';
    loadOrders();
  }, []);

  const loadOrders = async () => {
    try {
      const data = await pedidoService.listar();
      setOrders(data);
    } catch (err) {
      console.error(err);
      toast.error('Erro ao buscar pedidos');
    } finally {
      setLoading(false);
    }
  };

  const handleCancelOrder = async (id) => {
    if (!window.confirm('Tem certeza que deseja cancelar este pedido?')) return;
    try {
      await pedidoService.cancelar(id);
      toast.success('Pedido cancelado com sucesso!');
      await loadOrders();
      if (selectedOrder && selectedOrder.id === id) {
        handleViewDetail(id);
      }
    } catch (err) {
      const msg = err.response?.data?.erro || 'Erro ao cancelar pedido';
      toast.error(msg);
    }
  };

  const handleViewDetail = async (id) => {
    try {
      const detail = await pedidoService.detalhar(id);
      setSelectedOrder(detail);
    } catch (err) {
      toast.error('Erro ao carregar detalhes do pedido');
    }
  };

  const getStatusColor = (status) => {
    switch (status) {
      case 'pendente': return 'var(--warning)';
      case 'confirmado': return 'var(--info)';
      case 'enviado': return 'var(--accent-primary)';
      case 'entregue': return 'var(--success)';
      case 'cancelado': return 'var(--danger)';
      default: return 'var(--text-muted)';
    }
  };

  return (
    <div className="container animate-fade-in">
      <h1 style={{ fontSize: '2rem', fontWeight: 800, marginBottom: '32px' }}>Meus Pedidos</h1>

      <div style={{ display: 'flex', gap: '32px', flexWrap: 'wrap', alignItems: 'flex-start' }}>
        {}
        <div style={{ flex: 1, minWidth: '320px' }}>
          {loading ? (
            <div style={{ display: 'flex', justifyContent: 'center', padding: '40px' }}><div className="loading-spinner"></div></div>
          ) : orders.length === 0 ? (
            <div className="glass" style={{ padding: '40px', borderRadius: 'var(--radius-lg)', textAlign: 'center', color: 'var(--text-secondary)' }}>
              Nenhum pedido realizado ainda.
            </div>
          ) : (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
              {orders.map((order) => (
                <div key={order.id} className="glass" style={{ padding: '20px', borderRadius: 'var(--radius-md)', display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexWrap: 'wrap', gap: '12px' }}>
                  <div>
                    <h3 style={{ fontWeight: 700 }}>Pedido #{order.id}</h3>
                    <p style={{ fontSize: '0.85rem', color: 'var(--text-secondary)', display: 'flex', alignItems: 'center', gap: '6px', marginTop: '4px' }}>
                      <FaCalendarAlt /> {new Date(order.criado_em).toLocaleDateString('pt-BR')}
                    </p>
                    <p style={{ fontWeight: 800, color: 'var(--accent-secondary)', marginTop: '8px' }}>R$ {order.valor_total.toFixed(2)}</p>
                  </div>

                  <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                    <span
                      style={{
                        padding: '4px 10px',
                        borderRadius: 'var(--radius-sm)',
                        fontSize: '0.8rem',
                        fontWeight: 700,
                        textTransform: 'uppercase',
                        background: `${getStatusColor(order.status)}22`,
                        color: getStatusColor(order.status),
                        border: `1px solid ${getStatusColor(order.status)}44`
                      }}
                    >
                      {order.status}
                    </span>

                    <button onClick={() => handleViewDetail(order.id)} className="btn btn-secondary" style={{ padding: '6px 12px', fontSize: '0.85rem' }}>
                      Ver Detalhes
                    </button>

                    {(order.status === 'pendente' || order.status === 'confirmado') && (
                      <button onClick={() => handleCancelOrder(order.id)} className="btn btn-danger" style={{ padding: '6px 12px', fontSize: '0.85rem' }}>
                        Cancelar
                      </button>
                    )}
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {}
        {selectedOrder && (
          <div style={{ flex: 1, minWidth: '320px' }}>
            <div className="glass animate-fade-in" style={{ padding: '28px', borderRadius: 'var(--radius-lg)' }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '20px', borderBottom: '1px solid var(--border-color)', paddingBottom: '12px' }}>
                <h3 style={{ fontSize: '1.3rem', fontWeight: 700 }}>Detalhes do Pedido #{selectedOrder.id}</h3>
                <span
                  style={{
                    padding: '4px 10px',
                    borderRadius: 'var(--radius-sm)',
                    fontSize: '0.8rem',
                    fontWeight: 700,
                    textTransform: 'uppercase',
                    background: `${getStatusColor(selectedOrder.status)}22`,
                    color: getStatusColor(selectedOrder.status),
                    border: `1px solid ${getStatusColor(selectedOrder.status)}44`
                  }}
                >
                  {selectedOrder.status}
                </span>
              </div>

              <div style={{ display: 'flex', flexDirection: 'column', gap: '16px', marginBottom: '24px' }}>
                {selectedOrder.itens.map((item, idx) => (
                  <div key={idx} style={{ display: 'flex', justifyContent: 'space-between', fontSize: '0.95rem' }}>
                    <div>
                      <h4 style={{ fontWeight: 600 }}>{item.produto_nome}</h4>
                      <span style={{ fontSize: '0.85rem', color: 'var(--text-secondary)' }}>
                        {item.quantidade}x R$ {item.preco_unitario.toFixed(2)}
                      </span>
                    </div>
                    <span style={{ fontWeight: 700 }}>R$ {item.subtotal.toFixed(2)}</span>
                  </div>
                ))}
              </div>

              <div style={{ display: 'flex', justifyContent: 'space-between', borderTop: '1px solid var(--border-color)', paddingTop: '16px', fontSize: '1.25rem', fontWeight: 800 }}>
                <span>Valor Total</span>
                <span style={{ color: 'var(--accent-secondary)' }}>R$ {selectedOrder.valor_total.toFixed(2)}</span>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
