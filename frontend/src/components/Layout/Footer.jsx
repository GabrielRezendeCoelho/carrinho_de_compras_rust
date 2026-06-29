import React from 'react';

export const Footer = () => {
  return (
    <footer className="footer glass">
      <div className="container footer-container">
        <p>&copy; {new Date().getFullYear()} TechStore. Todos os direitos reservados.</p>
        <p>Sistema Full Stack desenvolvido em Rust (Rocket + Diesel + SQLite) & React.</p>
      </div>
    </footer>
  );
};
