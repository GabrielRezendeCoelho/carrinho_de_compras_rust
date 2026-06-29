import React, { createContext, useState, useEffect, useContext } from 'react';
import { carrinhoService } from '../services/carrinhoService';
import { useAuth } from './AuthContext';
import toast from 'react-hot-toast';

const CartContext = createContext(null);

export const CartProvider = ({ children }) => {
  const { user } = useAuth();
  const [cart, setCart] = useState(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (user) {
      loadCart();
    } else {
      setCart(null);
    }
  }, [user]);

  const loadCart = async () => {
    setLoading(true);
    try {
      const data = await carrinhoService.obter();
      setCart(data);
    } catch (err) {
      console.error("Erro ao carregar carrinho:", err);
    } finally {
      setLoading(false);
    }
  };

  const addItem = async (produtoId, quantidade) => {
    if (!user) {
      toast.error('Você precisa fazer login para comprar');
      return;
    }
    setLoading(true);
    try {
      await carrinhoService.adicionar(produtoId, quantidade);
      await loadCart();
      toast.success('Item adicionado ao carrinho!');
    } catch (err) {
      const msg = err.response?.data?.erro || 'Erro ao adicionar item';
      toast.error(msg);
    } finally {
      setLoading(false);
    }
  };

  const updateQuantity = async (itemId, quantidade) => {
    setLoading(true);
    try {
      await carrinhoService.atualizarQuantidade(itemId, quantidade);
      await loadCart();
      toast.success('Quantidade atualizada!');
    } catch (err) {
      const msg = err.response?.data?.erro || 'Erro ao atualizar quantidade';
      toast.error(msg);
    } finally {
      setLoading(false);
    }
  };

  const removeItem = async (itemId) => {
    setLoading(true);
    try {
      await carrinhoService.remover(itemId);
      await loadCart();
      toast.success('Item removido do carrinho');
    } catch (err) {
      toast.error('Erro ao remover item');
    } finally {
      setLoading(false);
    }
  };

  const clearCart = async () => {
    setLoading(true);
    try {
      await carrinhoService.limpar();
      await loadCart();
      toast.success('Carrinho limpo');
    } catch (err) {
      toast.error('Erro ao limpar carrinho');
    } finally {
      setLoading(false);
    }
  };

  return (
    <CartContext.Provider value={{ cart, loading, addItem, updateQuantity, removeItem, clearCart, loadCart }}>
      {children}
    </CartContext.Provider>
  );
};

export const useCart = () => useContext(CartContext);
