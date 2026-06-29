import React, { useState, useEffect } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { useAuth } from '../contexts/AuthContext';
import toast from 'react-hot-toast';

export const Login = () => {
  const { login } = useAuth();
  const navigate = useNavigate();
  const [email, setEmail] = useState('');
  const [senha, setSenha] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    document.title = 'Entrar — TechStore';
  }, []);

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (!email || !senha) {
      toast.error('Preencha todos os campos!');
      return;
    }

    setLoading(true);
    try {
      await login(email, senha);
      toast.success('Bem-vindo de volta!');
      navigate('/');
    } catch (err) {
      const msg = err.response?.data?.erro || 'Email ou senha inválidos';
      toast.error(msg);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="container" style={{ display: 'flex', minHeight: '80vh', alignItems: 'center' }}>
      <div className="form-card glass animate-fade-in">
        <h1 className="form-title" id="login-heading">Login</h1>
        <p className="form-subtitle">Acesse sua conta para gerenciar seu carrinho</p>

        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label className="form-label" htmlFor="email-input">E-mail</label>
            <input
              id="email-input"
              type="email"
              className="form-input"
              placeholder="seuemail@exemplo.com"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              disabled={loading}
              required
            />
          </div>

          <div className="form-group">
            <label className="form-label" htmlFor="senha-input">Senha</label>
            <input
              id="senha-input"
              type="password"
              className="form-input"
              placeholder="Sua senha secreta"
              value={senha}
              onChange={(e) => setSenha(e.target.value)}
              disabled={loading}
              required
            />
          </div>

          <button
            type="submit"
            className="btn btn-primary btn-block"
            disabled={loading}
            id="login-submit-btn"
          >
            {loading ? 'Entrando...' : 'Entrar na Conta'}
          </button>
        </form>

        <p className="form-text-link">
          Não tem uma conta? <Link to="/cadastro">Cadastre-se</Link>
        </p>
      </div>
    </div>
  );
};
