import React from 'react';
import './styleA.css';

const Dashboard = ({ currentPage, setCurrentPage }) => {
  return (
    <div className="dashboard">
      <div
        className={`dashboard-link ${currentPage === 'alpha' && 'active'}`}
        onClick={() => setCurrentPage('alpha')}
      >
        Alpha
      </div>
      <div
        className={`dashboard-link ${currentPage === 'beta' && 'active'}`}
        onClick={() => setCurrentPage('beta')}
      >
        Beta
      </div>
    </div>
  );
};

export default Dashboard;