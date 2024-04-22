import { open, save } from "@tauri-apps/api/dialog";
import { readTextFile, writeFile } from "@tauri-apps/api/fs";
import { appWindow } from "@tauri-apps/api/window";
import React, { useEffect, useState } from "react";
import "./Alpha.css";
import "./styleA.css";

const Alpha = () => {
  const [content, setContent] = useState("");
  // eslint-disable-next-line no-unused-vars
  const [isMounted, setIsMounted] = useState(true);

  useEffect(() => {
    const handleNewContent = () => {
      console.log("new-content event emitted");
      setContent("");
    };

    const handleOpenFile = async () => {
      try {
        const filePath = await open({
          title: "Select a Text File",
          filters: [{ name: "Text", extensions: ["txt"] }],
        });
        if (!filePath) return;
        const fileContent = await readTextFile(filePath, {});
        setContent(fileContent);
      } catch (error) {
        console.error(error);
      }
    };

    const handleSaveFile = async () => {
      try {
        const filePath = await save({
          title: "Save the Test file",
          filters: [{ name: "Text", extensions: ["txt"] }],
        });
        if (!filePath) return;

        await writeFile(filePath, content);
        console.log("File saved successfully");
      } catch (error) {
        console.log(error);
      }
    };

    appWindow.listen("new-content", handleNewContent);
    appWindow.listen("open-file", handleOpenFile);
    appWindow.listen("save-content", handleSaveFile);

    return () => {
      // Set the flag to false when the component is unmounted
      setIsMounted(false);
    };
  }, [content]);

  const handleTextareaChange = (event) => {
    setContent(event.target.value);
  };

  return (
    <>
      <div className="alpha-page">
        <div className="alpha-content">
          <h1 className="alpha-heading">Alpha Page</h1>
          <div className="alpha-description">
            Manage text files effortlessly:
            <ul>
              <li>Create, open, and save text files.</li>
              <li>Keyboard shortcuts for quick navigation between pages.</li>
              <li>Ctrl+B for more options.</li>
            </ul>
          </div>
        </div>
        <br />
        <div className="notes-heading">NoteBliz</div>
        <div className="text-area">
          <textarea
            value={content}
            onChange={handleTextareaChange}
            rows="10"
            cols="100"
            className="content-area"
            placeholder="Start typing here..."
          />
        </div>
      </div>
    </>
  );
};

export default Alpha;
