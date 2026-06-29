import api from './api';

export const pedidoService = {
  async criar() {
    const response = await api.post('/pedidos');
    return response.data;
  },

  async listar() {
    const response = await api.get('/pedidos');
    return response.data;
  },

  async detalhar(id) {
    const response = await api.get(`/pedidos/${id}`);
    return response.data;
  },

  async cancelar(id) {
    const response = await api.put(`/pedidos/${id}/cancelar`);
    return response.data;
  },
};
