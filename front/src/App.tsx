
import './App.css'
import LoginButton from "./components/Login.tsx";
import Profile from "./components/Profile.tsx";
import LogoutButton from "./components/Logout.tsx";
import GetUser from "./components/GetUser.tsx";

function App() {

  return (
    <>
      <LoginButton />
      <LogoutButton />
      <Profile />
      <GetUser />
    </>
  )
}

export default App
