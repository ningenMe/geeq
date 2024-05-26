"use client";

import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import Toolbar from "@mui/material/Toolbar";
import IconButton from "@mui/material/IconButton";
import Typography from "@mui/material/Typography";
import Menu from "@mui/material/Menu";
import Container from "@mui/material/Container";
import PersonIcon from "@mui/icons-material/Person";
import MenuItem from "@mui/material/MenuItem";
import CodeIcon from "@mui/icons-material/Code";
import { useEffect, useState } from "react";
import { GITHUB_CLIENT_ID } from "./constant";
import { geeqApiClient } from "./client/GeeqApiClient";
import { useRouter } from "next/navigation";

export const Header = () => {
  const productName = "GEEQ";
  const redirectUri = process.env.NEXT_PUBLIC_FRONT_ORIGIN + "/auth/callback";
  const githubOauthUrl = `https://github.com/login/oauth/authorize?client_id=${GITHUB_CLIENT_ID}&redirect_uri=${redirectUri}`;
  const router = useRouter();

  const [loginUserId, setLoginUserId] = useState<string | null>(null);
  const [anchorElUser, setAnchorElUser] = useState<null | HTMLElement>(null);

  useEffect(() => {
    geeqApiClient
      .authMeGet({
        withCredentials: true,
      })
      .then((res) => {
        setLoginUserId(res.data.userId);
      })
      .catch(() => {
        setLoginUserId(null);
      });
  });

  const settings = [
    {
      name: "マイページ",
      isDisplayed: !!loginUserId,
      onClick: () => {
        router.push("/user/" + loginUserId);
      },
    },
    {
      name: "GitHubログイン",
      isDisplayed: !loginUserId,
      onClick: () => {
        router.push(githubOauthUrl);
      },
    },
    {
      name: "ログアウト",
      href: "/auth/logout",
      isDisplayed: !!loginUserId,
      onClick: () => {
        router.push("/auth/logout");
      },
    },
  ];

  const handleOpenUserMenu = (event: React.MouseEvent<HTMLElement>) => {
    setAnchorElUser(event.currentTarget);
  };
  const handleCloseUserMenu = () => {
    setAnchorElUser(null);
  };

  return (
    <AppBar position="static">
      <Container maxWidth="xl">
        <Toolbar disableGutters>
          <CodeIcon sx={{ display: { xs: "flex", md: "flex" }, mr: 1 }} />
          <Typography
            variant="h5"
            noWrap
            component="a"
            href="/"
            sx={{
              mr: 2,
              display: { xs: "flex", md: "flex" },
              flexGrow: 1,
              fontFamily: "monospace",
              fontWeight: 700,
              letterSpacing: ".3rem",
              color: "inherit",
              textDecoration: "none",
            }}
          >
            {productName}
          </Typography>

          <Box sx={{ flexGrow: 0 }}>
            <IconButton onClick={handleOpenUserMenu} sx={{ p: 0 }}>
              <PersonIcon fontSize="large" />
              {loginUserId}
            </IconButton>
            <Menu
              sx={{ mt: "45px" }}
              id="menu-appbar"
              anchorEl={anchorElUser}
              anchorOrigin={{
                vertical: "top",
                horizontal: "right",
              }}
              keepMounted
              transformOrigin={{
                vertical: "top",
                horizontal: "right",
              }}
              open={Boolean(anchorElUser)}
              onClose={handleCloseUserMenu}
            >
              {settings
                .filter(({ isDisplayed }) => isDisplayed)
                .map(({ name, onClick }) => (
                  <MenuItem key={name} onClick={handleCloseUserMenu}>
                    <Typography
                      textAlign="center"
                      component="a"
                      onClick={onClick}
                    >
                      {name}
                    </Typography>
                  </MenuItem>
                ))}
            </Menu>
          </Box>
        </Toolbar>
      </Container>
    </AppBar>
  );
};
