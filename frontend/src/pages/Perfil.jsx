import React from 'react';
import { useAuth } from '../contexts/AuthContext';
import { FaUser, FaEnvelope, FaCalendarAlt, FaUserShield } from 'react-icons/fa';

export const Perfil = () => {
  const { user } = useAuth();

  React.useEffect(() => {
    document.title = 'Meu Perfil — TechStore';
  }, []);

  if (!user) return null;

  return (
    <div className="container animate-fade-in" style={{ maxWidth: '600px', marginTop: '40px' }}>
      <div className="glass" style={{ padding: '40px', borderRadius: 'var(--radius-lg)' }}>
        <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '16px', marginBottom: '32px', borderBottom: '1px solid var(--border-color)', paddingBottom: '24px' }}>
          <div style={{ width: '80px', height: '80px', borderRadius: 'var(--radius-full)', background: 'var(--accent-gradient)', display: 'flex', alignItems: 'center', justify: 'center', color: 'white', fontSize: '2.5rem' }}>
            <FaUser />
          </div>
          <h2 style={{ fontSize: '1.8rem', fontWeight: 800 }}>{user.nome}</h2>
          {user.email === 'admin@shop.com' && (
            <span style={{ display: 'flex', alignItems: 'center', gap: '6px', background: 'rgba(99, 102, 241, 0.1)', color: 'var(--accent-primary)', padding: '6px 12px', borderRadius: 'var(--radius-sm)', fontSize: '0.85rem', fontWeight: 700 }}>
              <FaUserShield /> Administrador
            </span>
          )}
        </div>

        <div style={{ display: 'flex', flexDirection: 'column', gap: '20px' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
            <FaEnvelope style={{ color: 'var(--accent-secondary)', fontSize: '1.2rem' }} />
            <div>
              <span style={{ display: 'block', fontSize: '0.8rem', color: 'var(--text-secondary)', textTransform: 'uppercase', fontWeight: 600 }}>E-mail</span>
              <span style={{ fontSize: '1.1rem', fontWeight: 500 }}>{user.email}</span>
            </div>
          </div>

          <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
            <FaCalendarAlt style={{ color: 'var(--accent-primary)', fontSize: '1.2rem' }} />
            <div>
              <span style={{ display: 'block', fontSize: '0.8rem', color: 'var(--text-secondary)', textTransform: 'uppercase', fontWeight: 600 }}>Membro desde</span>
              <span style={{ fontSize: '1.1rem', fontWeight: 500 }}>
                {new Date(user.criado_em).toLocaleDateString('pt-BR')}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
