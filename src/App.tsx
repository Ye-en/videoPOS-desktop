import { BrowserRouter, Route, Routes } from "react-router-dom";
import "./App.css";
import SetupPage from "./page/Setup";
import HomePage from "./page/Home";
import LoadingPage from "./page/Loading";


function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<LoadingPage />} />
        <Route path="/home" element={<HomePage />} />
        <Route path="/setup" element={<SetupPage />} />
      </Routes>
    </BrowserRouter>
  )
}

export default App;
