import React, { useState } from 'react';
import './ActivityBar.css';

const ActivityBar = ({ currentPage, setCurrentPage }) => {
  const [showFilesDropdown, setShowFilesDropdown] = useState(false);

  const toggleFilesDropdown = () => {
    setShowFilesDropdown(!showFilesDropdown);
  };

  const openAlphaPage = () => {
    setCurrentPage('alpha');
  };

  const openBetaPage = () => {
    setCurrentPage('beta');
  };

  return (
    <div className="activity-bar">
      <div className="files-option file-title" onClick={toggleFilesDropdown}>
        Files
        {showFilesDropdown && (
          <div className="files-dropdown">
            <div className="files-options" onClick={openAlphaPage}>Alpha</div>
            <div className="files-options" onClick={openBetaPage}>Beta</div>
          </div>
        )}
      </div>
      <div className="files-option file-title">Tools</div>
      <div className="files-option file-title">Edit</div>
    </div>
  );
};

export default ActivityBar;

      /* <div className={`page-button ${currentPage === 'alpha' ? 'active' : ''}`} onClick={openAlphaPage}>
        Alpha
      </div>
      <div className={`page-button ${currentPage === 'beta' ? 'active' : ''}`} onClick={openBetaPage}>
        Beta
      </div> */