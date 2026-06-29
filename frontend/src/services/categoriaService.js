import api from './api';

export const categoriaService = {
  async listar() {
    const response = await api.get('/categorias');
    return response.data;
  },

  async buscarPorId(id) {
    const response = await api.get(`/categorias/${id}`);
    return response.data;
  },

  async criar(dados) {
    const response = await api.post('/categorias', dados);
    return response.data;
  },

  async atualizar(id, dados) {
    const response = await api.put(`/categorias/${id}`, dados);
    return response.data;
  },

  async remover(id) {
    const response = await api.delete(`/categorias/${id}`);
    return response.data;
  },
};
