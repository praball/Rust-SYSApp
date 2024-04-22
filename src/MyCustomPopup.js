import React from 'react';
import ReactPopup from 'reactjs-popup'; // Rename the import to avoid conflicts
import './styleA.css';

const MyCustomPopup = ({ onClose, setCurrentPage }) => {
  const handlePageChange = (page) => {
    setCurrentPage(page);
    onClose();
  };

  return (
    <ReactPopup open={true} position="right center" onClose={onClose}>
      <div className="popup-content">
        <div className='popup-heading'>Choose a page!</div><br/>
        <button className="popup-button" onClick={() => handlePageChange('alpha')}>
          Go to Alpha
        </button>
        <button className="popup-button" onClick={() => handlePageChange('beta')}>
          Go to Beta
        </button>
      </div>
    </ReactPopup>
  );
};

export default MyCustomPopup;