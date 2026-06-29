import api from './api';

export const authService = {
  async register(nome, email, senha) {
    const response = await api.post('/auth/register', { nome, email, senha });
    return response.data;
  },

  async login(email, senha) {
    const response = await api.post('/auth/login', { email, senha });
    return response.data;
  },

  async me() {
    const response = await api.get('/auth/me');
    return response.data;
  },
};
