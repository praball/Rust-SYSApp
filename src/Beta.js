import React from "react";
import "./Beta.css";
import "./styleA.css";

const Beta = () => {
  return (
    <>
      <div className="beta-page">
        <div className="beta-content">
          <h1 className="beta-heading">Beta Page</h1>
          <div className="beta-description">
            This is the second page. To switch between pages, hit <b>Ctrl+b</b>.
          </div>
        </div>
      </div>
    </>
  );
};

export default Beta;
