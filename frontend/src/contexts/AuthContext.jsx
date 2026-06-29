import React, { createContext, useState, useEffect, useContext } from 'react';
import { authService } from '../services/authService';

const AuthContext = createContext(null);

export const AuthProvider = ({ children }) => {
  const [user, setUser] = useState(null);
  const [token, setToken] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const loadStorage = async () => {
      const storedToken = localStorage.getItem('shopping_cart_token');
      const storedUser = localStorage.getItem('shopping_cart_user');

      if (storedToken && storedUser) {
        setToken(storedToken);
        setUser(JSON.parse(storedUser));

        try {
          const profile = await authService.me();
          setUser(profile);
          localStorage.setItem('shopping_cart_user', JSON.stringify(profile));
        } catch (err) {
          logout();
        }
      }
      setLoading(false);
    };

    loadStorage();
  }, []);

  const login = async (email, senha) => {
    setLoading(true);
    try {
      const data = await authService.login(email, senha);
      setToken(data.token);
      setUser(data.usuario);
      localStorage.setItem('shopping_cart_token', data.token);
      localStorage.setItem('shopping_cart_user', JSON.stringify(data.usuario));
      return data.usuario;
    } finally {
      setLoading(false);
    }
  };

  const register = async (nome, email, senha) => {
    setLoading(true);
    try {
      const data = await authService.register(nome, email, senha);
      setToken(data.token);
      setUser(data.usuario);
      localStorage.setItem('shopping_cart_token', data.token);
      localStorage.setItem('shopping_cart_user', JSON.stringify(data.usuario));
      return data.usuario;
    } finally {
      setLoading(false);
    }
  };

  const logout = () => {
    setToken(null);
    setUser(null);
    localStorage.removeItem('shopping_cart_token');
    localStorage.removeItem('shopping_cart_user');
  };

  const isAdmin = () => {
    return user && user.email === 'admin@shop.com';
  };

  return (
    <AuthContext.Provider value={{ user, token, loading, login, register, logout, isAdmin }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => useContext(AuthContext);
