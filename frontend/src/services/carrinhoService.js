import api from './api';

export const carrinhoService = {
  async obter() {
    const response = await api.get('/carrinho');
    return response.data;
  },

  async adicionar(produtoId, quantidade) {
    const response = await api.post('/carrinho/itens', { produto_id: produtoId, quantidade });
    return response.data;
  },

  async atualizarQuantidade(itemId, quantidade) {
    const response = await api.put(`/carrinho/itens/${itemId}`, { quantidade });
    return response.data;
  },

  async remover(itemId) {
    const response = await api.delete(`/carrinho/itens/${itemId}`);
    return response.data;
  },

  async limpar() {
    const response = await api.delete('/carrinho');
    return response.data;
  },
};
