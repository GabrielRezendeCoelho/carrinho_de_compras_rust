import React, { useState, useEffect } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { useAuth } from '../contexts/AuthContext';
import toast from 'react-hot-toast';

export const Cadastro = () => {
  const { register } = useAuth();
  const navigate = useNavigate();
  const [nome, setNome] = useState('');
  const [email, setEmail] = useState('');
  const [senha, setSenha] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    document.title = 'Criar Conta — TechStore';
  }, []);

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (!nome || !email || !senha) {
      toast.error('Preencha todos os campos!');
      return;
    }

    if (senha.length < 6) {
      toast.error('A senha deve ter pelo menos 6 caracteres!');
      return;
    }

    setLoading(true);
    try {
      await register(nome, email, senha);
      toast.success('Cadastro realizado com sucesso!');
      navigate('/');
    } catch (err) {
      const msg = err.response?.data?.erro || 'Erro ao realizar cadastro';
      toast.error(msg);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="container" style={{ display: 'flex', minHeight: '80vh', alignItems: 'center' }}>
      <div className="form-card glass animate-fade-in">
        <h1 className="form-title" id="cadastro-heading">Criar Conta</h1>
        <p className="form-subtitle">Cadastre-se para aproveitar ofertas imperdíveis</p>

        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label className="form-label" htmlFor="nome-input">Nome Completo</label>
            <input
              id="nome-input"
              type="text"
              className="form-input"
              placeholder="Digite seu nome"
              value={nome}
              onChange={(e) => setNome(e.target.value)}
              disabled={loading}
              required
            />
          </div>

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
            <label className="form-label" htmlFor="senha-input">Senha (mínimo 6 caracteres)</label>
            <input
              id="senha-input"
              type="password"
              className="form-input"
              placeholder="Crie uma senha forte"
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
            id="cadastro-submit-btn"
          >
            {loading ? 'Cadastrando...' : 'Finalizar Cadastro'}
          </button>
        </form>

        <p className="form-text-link">
          Já tem uma conta? <Link to="/login">Faça Login</Link>
        </p>
      </div>
    </div>
  );
};
