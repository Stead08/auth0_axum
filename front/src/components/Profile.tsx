import { useEffect, useState } from "react";
import { useAuth0 } from "@auth0/auth0-react";

const Profile = () => {
  const { user, isAuthenticated, getAccessTokenSilently } = useAuth0();
  const [userMetadata, setUserMetadata] = useState(null);

  useEffect(() => {
    const getUserMetadata = async () => {
      const domain = "dev-jzar8fywnhduze62.us.auth0.com";
      if (!user) {
        return;
      }
      try {
        const accessToken = await getAccessTokenSilently({
          authorizationParams: {
            audience: `https://${domain}/api/v2/`,
            scope: "read:current_user",
          },
        });
        const userDetailsByIdUrl = `https://${domain}/api/v2/users/${user.sub}`;
        console.log(`fetching metadata from ${userDetailsByIdUrl}`);
        const metadataResponse = await fetch(userDetailsByIdUrl, {
          headers: {
            Authorization: `Bearer ${accessToken}`,
          },
        });

        console.log(metadataResponse);
        const { user_metadata } = await metadataResponse.json();
        console.log(user_metadata);
        setUserMetadata(user_metadata);
      } catch (e) {
        console.log(e);
      }
    };
    getUserMetadata().catch((e) => console.error(e))
  }, [getAccessTokenSilently, user?.sub]);

  return (
      isAuthenticated && (
          <div>
            <img src={user?.picture} alt={user?.name} />
            <h2>{user?.name}</h2>
            <p>{user?.email}</p>
            <h3>User Metadata</h3>
            {userMetadata ? (
                <pre>{JSON.stringify(userMetadata, null, 2)}</pre>
            ) : (
                "No user metadata defined"
            )}
          </div>
      )
  );
};

export default Profile;