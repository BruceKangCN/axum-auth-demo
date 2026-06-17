import { ref } from "vue";
import { defineStore } from "pinia";
import Cookies from "js-cookie";
import ky from "ky";

interface User {
  sub: string;
  username: string;
  accessToken: string;
}

interface UserInfo {
  sub: string;
  preferred_username: string;
}

interface RefreshResponse {
  accessToken: string;
}

export const useUserStore = defineStore("user", () => {
  const user = ref<User | null>(null);

  // Update user info from access token
  async function updateUser() {
    const accessTokenKey = "oauth2.access-token";

    const accessToken = Cookies.get(accessTokenKey) ?? (await refreshTokens());
    Cookies.remove(accessTokenKey);

    if (!accessToken) {
      console.error("update user failed : missing access token");
      return;
    }

    user.value = await getUser(accessToken);
  }

  async function refresh() {
    const accessToken = await refreshTokens();
    if (!accessToken) {
      console.error("failed to refresh tokens");
      return;
    }

    if (user.value) {
      user.value.accessToken = accessToken;
      return;
    }

    user.value = await getUser(accessToken);
  }

  return { user, updateUser, refresh };
});

async function refreshTokens() {
  console.log("refresh tokens");
  try {
    const response: RefreshResponse = await ky.post("/api/auth/refresh").json();
    return response.accessToken;
  } catch (error) {
    console.error({ msg: "Failed to fetch access token", error });
    return;
  }
}

async function getUserInfo(accessToken: string) {
  try {
    console.log("get user info");
    const userInfo: UserInfo = await ky
      .get("/authentik/application/o/userinfo/", {
        headers: {
          Authorization: `Bearer ${accessToken}`,
        },
      })
      .json();
    return userInfo;
  } catch (error) {
    console.error({ msg: "Failed to fetch user info", error });
  }
}

async function getUser(accessToken: string) {
  const userInfo = await getUserInfo(accessToken);
  if (!userInfo) {
    console.warn("failed to get user info");
    return null;
  }

  return {
    sub: userInfo.sub,
    username: userInfo.preferred_username,
    accessToken,
  } as User;
}
