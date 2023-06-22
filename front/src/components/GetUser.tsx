import React, {useState} from "react";
import {useAuth0} from "@auth0/auth0-react";

const GetUser = () => {
  const { user, isAuthenticated, getAccessTokenSilently } = useAuth0();
  const [userList, setUserList] = useState("null");

  const OnClickHandler = async (e: React.MouseEvent<HTMLButtonElement>) => {
    if (!isAuthenticated) {
      return;
    }
    e.preventDefault();
    const domain = "dev-jzar8fywnhduze62.us.auth0.com";
    if (!user) {
      return;
    }
    try {
      const accessToken = await getAccessTokenSilently({
        authorizationParams: {
          audience: `https://${domain}/api/v2/`,
        },
      });
      const userDetailsByIdUrl = `http://localhost:8080/api/users`;
      console.log(`fetching from ${userDetailsByIdUrl}`);
      const metadataResponse = await fetch(userDetailsByIdUrl, {

        headers: {
          Authorization: `Bearer ${accessToken}`,
        },
      });

      const users = await metadataResponse.json();
      setUserList(JSON.stringify(users));
    } catch (e) {
      console.log(e);
    }
  }

  return(
      <>
        <button onClick={(e) => OnClickHandler(e) } >Get User</button>
        <h5>{userList}</h5>
      </>
  )
}

export default GetUser;