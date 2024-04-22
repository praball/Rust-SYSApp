// App.js
import { appWindow } from "@tauri-apps/api/window";
import React, { useEffect, useState } from "react";
import ActivityBar from "./ActivityBar";
import Alpha from "./Alpha";
import Beta from "./Beta";
import MyPopup from "./MyCustomPopup";
import "./styleA.css";

const App = () => {
  const [currentPage, setCurrentPage] = useState("alpha");
  const [showPopup, setShowPopup] = useState(false);

  useEffect(() => {
    const handleShortcut = (event) => {
      if (event.key === "b" && (event.metaKey || event.ctrlKey)) {
        // setCurrentPage('alpha');
        setShowPopup(true);
      }
      else if (event.key === "i" && (event.metaKey || event.ctrlKey)) {
        setCurrentPage('alpha');
      }
      else if (event.key === 't' && (event.metaKey || event.ctrlKey)) {
        setCurrentPage('beta');
      }
      // else if (event.key === 'b' && (event.metaKey || event.ctrlKey)) {
      //   // setCurrentPage('beta');
      //   setShowPopup(true);
      // }
    };

    const unlisten = async () => {
      await appWindow.listen("switch-page", (event) => {
        setCurrentPage(event.payload);
      });
    };

    window.addEventListener("keydown", handleShortcut);
    unlisten();

    return () => {
      window.removeEventListener("keydown", handleShortcut);
    };
  }, [currentPage]); // Include currentPage as a dependency

  const closePopup = () => {
    setShowPopup(false);
  };

  return (
    <>
      <div>
        {/* <Dashboard currentPage={currentPage} setCurrentPage={setCurrentPage} /> */}
        <ActivityBar
          currentPage={currentPage}
          setCurrentPage={setCurrentPage}
        />
        <div className="page-content">
          {currentPage === "alpha" ? <Alpha /> : <Beta />}
        </div>
        {showPopup && (
          <MyPopup onClose={closePopup} setCurrentPage={setCurrentPage} />
        )}
      </div>
      <br />
    </>
  );
};

export default App;

// import React from 'react';
// import Alpha from './Alpha';
// import Beta from './Beta';

// const App = () => {
//   const currentWindow = window.__TAURI__.window.label;

//   return (
//     <div>
//       {currentWindow === 'alpha' ? <Alpha /> : <Beta />}
//     </div>
//   );
// };

// export default App;
