import React from 'react';
import { Routes, Route } from 'react-router-dom';
import { Home } from '../pages/Home';
import { Login } from '../pages/Login';
import { Cadastro } from '../pages/Cadastro';
import { Produtos } from '../pages/Produtos';
import { Categorias } from '../pages/Categorias';
import { Carrinho } from '../pages/Carrinho';
import { Checkout } from '../pages/Checkout';
import { Pedidos } from '../pages/Pedidos';
import { Perfil } from '../pages/Perfil';
import { Dashboard } from '../pages/admin/Dashboard';
import { ProtectedRoute } from '../components/ProtectedRoute';

export const AppRoutes = () => {
  return (
    <Routes>
      {}
      <Route path="/" element={<Home />} />
      <Route path="/login" element={<Login />} />
      <Route path="/cadastro" element={<Cadastro />} />
      <Route path="/produtos" element={<Produtos />} />
      <Route path="/categorias" element={<Categorias />} />

      {}
      <Route
        path="/carrinho"
        element={
          <ProtectedRoute>
            <Carrinho />
          </ProtectedRoute>
        }
      />
      <Route
        path="/checkout"
        element={
          <ProtectedRoute>
            <Checkout />
          </ProtectedRoute>
        }
      />
      <Route
        path="/pedidos"
        element={
          <ProtectedRoute>
            <Pedidos />
          </ProtectedRoute>
        }
      />
      <Route
        path="/perfil"
        element={
          <ProtectedRoute>
            <Perfil />
          </ProtectedRoute>
        }
      />

      {}
      <Route
        path="/admin"
        element={
          <ProtectedRoute adminOnly={true}>
            <Dashboard />
          </ProtectedRoute>
        }
      />
    </Routes>
  );
};
