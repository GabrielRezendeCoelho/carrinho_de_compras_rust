import React from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../../contexts/AuthContext';
import { useCart } from '../../contexts/CartContext';
import { FaShoppingCart, FaUser, FaSignOutAlt, FaPlus, FaTasks } from 'react-icons/fa';

export const Navbar = () => {
  const { user, logout, isAdmin } = useAuth();
  const { cart } = useCart();
  const navigate = useNavigate();

  const totalItems = cart?.itens?.reduce((sum, item) => sum + item.quantidade, 0) || 0;

  return (
    <nav className="navbar glass">
      <div className="navbar-container container">
        <Link to="/" className="logo-text">
          TECH<span>STORE</span>
        </Link>

        <div className="nav-links">
          <Link to="/" className="nav-link">Home</Link>
          <Link to="/produtos" className="nav-link">Produtos</Link>
          <Link to="/categorias" className="nav-link">Categorias</Link>

          {isAdmin() && (
            <Link to="/admin" className="admin-badge nav-link">
              <FaTasks style={{ marginRight: '6px' }} /> Painel Admin
            </Link>
          )}
        </div>

        <div className="nav-actions">
          <Link to="/carrinho" className="cart-btn">
            <FaShoppingCart />
            {totalItems > 0 && <span className="cart-badge">{totalItems}</span>}
          </Link>

          {user ? (
            <div className="user-menu">
              <Link to="/perfil" className="user-btn">
                <FaUser style={{ marginRight: '6px' }} />
                <span className="user-name-text">{user.nome.split(' ')[0]}</span>
              </Link>
              <button onClick={() => { logout(); navigate('/login'); }} className="logout-btn">
                <FaSignOutAlt />
              </button>
            </div>
          ) : (
            <div className="auth-btns">
              <Link to="/login" className="login-btn">Entrar</Link>
              <Link to="/cadastro" className="register-btn">Cadastrar</Link>
            </div>
          )}
        </div>
      </div>
    </nav>
  );
};
