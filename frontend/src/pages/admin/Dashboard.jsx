import React, { useState, useEffect } from 'react';
import { produtoService } from '../../services/produtoService';
import { categoriaService } from '../../services/categoriaService';
import toast from 'react-hot-toast';
import { FaPlus, FaTrash, FaEdit, FaFolderPlus } from 'react-icons/fa';

export const Dashboard = () => {
  const [products, setProducts] = useState([]);
  const [categories, setCategories] = useState([]);
  const [loading, setLoading] = useState(true);

  const [prodNome, setProdNome] = useState('');
  const [prodDesc, setProdDesc] = useState('');
  const [prodPreco, setProdPreco] = useState('');
  const [prodEstoque, setProdEstoque] = useState('');
  const [prodCatId, setProdCatId] = useState('');

  const [catNome, setCatNome] = useState('');

  useEffect(() => {
    document.title = 'Painel Administrativo — TechStore';
    loadData();
  }, []);

  const loadData = async () => {
    setLoading(true);
    try {
      const prods = await produtoService.listar({ por_pagina: 100 });
      setProducts(prods.dados);

      const cats = await categoriaService.listar();
      setCategories(cats);
    } catch (err) {
      toast.error('Erro ao buscar dados do painel');
    } finally {
      setLoading(false);
    }
  };

  const handleAddCategory = async (e) => {
    e.preventDefault();
    if (!catNome) return;
    try {
      await categoriaService.criar({ nome: catNome });
      toast.success('Categoria criada!');
      setCatNome('');
      await loadData();
    } catch (err) {
      toast.error(err.response?.data?.erro || 'Erro ao criar categoria');
    }
  };

  const handleAddProduct = async (e) => {
    e.preventDefault();
    if (!prodNome || !prodPreco || !prodEstoque || !prodCatId) {
      toast.error('Preencha os campos obrigatórios!');
      return;
    }

    try {
      await produtoService.criar({
        categoria_id: parseInt(prodCatId, 10),
        nome: prodNome,
        descricao: prodDesc,
        preco: parseFloat(prodPreco),
        estoque: parseInt(prodEstoque, 10),
        imagem: null,
      });
      toast.success('Produto criado com sucesso!');
      setProdNome('');
      setProdDesc('');
      setProdPreco('');
      setProdEstoque('');
      setProdCatId('');
      await loadData();
    } catch (err) {
      toast.error(err.response?.data?.erro || 'Erro ao criar produto');
    }
  };

  const handleDeleteProduct = async (id) => {
    if (!window.confirm('Excluir este produto?')) return;
    try {
      await produtoService.remover(id);
      toast.success('Produto excluído');
      await loadData();
    } catch (err) {
      toast.error('Erro ao excluir produto');
    }
  };

  const handleDeleteCategory = async (id) => {
    if (!window.confirm('Excluir esta categoria? Todos os produtos vinculados poderão ser afetados.')) return;
    try {
      await categoriaService.remover(id);
      toast.success('Categoria excluída');
      await loadData();
    } catch (err) {
      toast.error('Erro ao excluir categoria. Verifique se existem produtos associados.');
    }
  };

  return (
    <div className="container animate-fade-in">
      <h1 style={{ fontSize: '2.2rem', fontWeight: 800, marginBottom: '40px' }}>Painel Administrativo</h1>

      {loading ? (
        <div style={{ display: 'flex', justifyContent: 'center', padding: '40px' }}><div className="loading-spinner"></div></div>
      ) : (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '40px' }}>

          {}
          <div style={{ display: 'flex', gap: '32px', flexWrap: 'wrap' }}>
            <div className="glass" style={{ flex: 1, minWidth: '300px', padding: '28px', borderRadius: 'var(--radius-lg)' }}>
              <h2 style={{ fontSize: '1.3rem', fontWeight: 700, marginBottom: '20px', display: 'flex', alignItems: 'center', gap: '10px' }}>
                <FaFolderPlus /> Nova Categoria
              </h2>
              <form onSubmit={handleAddCategory}>
                <div className="form-group">
                  <label className="form-label" htmlFor="cat-name">Nome da Categoria</label>
                  <input
                    id="cat-name"
                    type="text"
                    className="form-input"
                    placeholder="Ex: Acessórios"
                    value={catNome}
                    onChange={(e) => setCatNome(e.target.value)}
                    required
                  />
                </div>
                <button type="submit" className="btn btn-primary btn-block">Salvar Categoria</button>
              </form>
            </div>

            <div className="glass" style={{ flex: 2, minWidth: '320px', padding: '28px', borderRadius: 'var(--radius-lg)' }}>
              <h2 style={{ fontSize: '1.3rem', fontWeight: 700, marginBottom: '20px' }}>Categorias Cadastradas</h2>
              <div style={{ maxHeight: '200px', overflowY: 'auto', display: 'flex', flexDirection: 'column', gap: '10px' }}>
                {categories.map((cat) => (
                  <div key={cat.id} style={{ display: 'flex', justifyContent: 'space-between', padding: '12px', borderBottom: '1px solid var(--border-color)' }}>
                    <span>{cat.nome}</span>
                    <button onClick={() => handleDeleteCategory(cat.id)} style={{ color: 'var(--danger)', cursor: 'pointer' }}><FaTrash /></button>
                  </div>
                ))}
              </div>
            </div>
          </div>

          {}
          <div style={{ display: 'flex', gap: '32px', flexWrap: 'wrap' }}>
            {}
            <div className="glass" style={{ flex: 1, minWidth: '300px', padding: '28px', borderRadius: 'var(--radius-lg)' }}>
              <h2 style={{ fontSize: '1.3rem', fontWeight: 700, marginBottom: '20px', display: 'flex', alignItems: 'center', gap: '10px' }}>
                <FaPlus /> Novo Produto
              </h2>
              <form onSubmit={handleAddProduct}>
                <div className="form-group">
                  <label className="form-label" htmlFor="prod-name">Nome do Produto</label>
                  <input id="prod-name" type="text" className="form-input" placeholder="Ex: Teclado Mecânico" value={prodNome} onChange={(e) => setProdNome(e.target.value)} required />
                </div>
                <div className="form-group">
                  <label className="form-label" htmlFor="prod-desc">Descrição</label>
                  <textarea id="prod-desc" className="form-input" placeholder="Breve resumo sobre o produto" value={prodDesc} onChange={(e) => setProdDesc(e.target.value)} rows="3" />
                </div>
                <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
                  <div className="form-group">
                    <label className="form-label" htmlFor="prod-price">Preço (R$)</label>
                    <input id="prod-price" type="number" step="0.01" className="form-input" placeholder="199.90" value={prodPreco} onChange={(e) => setProdPreco(e.target.value)} required />
                  </div>
                  <div className="form-group">
                    <label className="form-label" htmlFor="prod-stock">Estoque</label>
                    <input id="prod-stock" type="number" className="form-input" placeholder="10" value={prodEstoque} onChange={(e) => setProdEstoque(e.target.value)} required />
                  </div>
                </div>
                <div className="form-group">
                  <label className="form-label" htmlFor="prod-cat">Categoria</label>
                  <select id="prod-cat" className="form-input" value={prodCatId} onChange={(e) => setProdCatId(e.target.value)} required>
                    <option value="">Selecione...</option>
                    {categories.map((cat) => (
                      <option key={cat.id} value={cat.id}>{cat.nome}</option>
                    ))}
                  </select>
                </div>
                <button type="submit" className="btn btn-primary btn-block">Salvar Produto</button>
              </form>
            </div>

            {}
            <div className="glass" style={{ flex: 2, minWidth: '320px', padding: '28px', borderRadius: 'var(--radius-lg)' }}>
              <h2 style={{ fontSize: '1.3rem', fontWeight: 700, marginBottom: '20px' }}>Produtos Cadastrados</h2>
              <div style={{ maxHeight: '420px', overflowY: 'auto' }}>
                <table style={{ width: '100%', borderCollapse: 'collapse', textAlign: 'left' }}>
                  <thead>
                    <tr style={{ borderBottom: '2px solid var(--border-color)', color: 'var(--text-secondary)', fontSize: '0.85rem' }}>
                      <th style={{ padding: '12px' }}>Produto</th>
                      <th style={{ padding: '12px' }}>Categoria</th>
                      <th style={{ padding: '12px' }}>Preço</th>
                      <th style={{ padding: '12px' }}>Estoque</th>
                      <th style={{ padding: '12px', textAlign: 'right' }}>Ações</th>
                    </tr>
                  </thead>
                  <tbody>
                    {products.map((prod) => (
                      <tr key={prod.id} style={{ borderBottom: '1px solid var(--border-color)' }}>
                        <td style={{ padding: '12px', fontWeight: 600 }}>{prod.nome}</td>
                        <td style={{ padding: '12px', color: 'var(--text-secondary)' }}>{prod.categoria_nome}</td>
                        <td style={{ padding: '12px' }}>R$ {prod.preco.toFixed(2)}</td>
                        <td style={{ padding: '12px' }}>{prod.estoque} un</td>
                        <td style={{ padding: '12px', textAlign: 'right' }}>
                          <button onClick={() => handleDeleteProduct(prod.id)} style={{ color: 'var(--danger)', cursor: 'pointer' }} title="Excluir"><FaTrash /></button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          </div>

        </div>
      )}
    </div>
  );
};
