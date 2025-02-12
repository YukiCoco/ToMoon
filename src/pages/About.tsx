import { FC } from "react";
import { ButtonItem, PanelSectionRow, Navigation } from "@decky/ui";
import { FiGithub } from "react-icons/fi";
import { FaSteamSymbol } from "react-icons/fa";
import { TbBrandTelegram } from "react-icons/tb";

export const About: FC = () => {
  return (
    // The outermost div is to push the content down into the visible area
    <>
      <h2
        style={{ fontWeight: "bold", fontSize: "1.5em", marginBottom: "0px" }}
      >
        To Moon
      </h2>
      <span>
        A network toolbox for SteamOS.
        <br />
      </span>
      <PanelSectionRow>
        <ButtonItem
          icon={<FiGithub style={{ display: "block" }} />}
          label="ToMoon"
          onClick={() => {
            Navigation.NavigateToExternalWeb(
              "https://github.com/YukiCoco/ToMoon"
            );
          }}
        >
          GitHub Repo
        </ButtonItem>
      </PanelSectionRow>
      <h2
        style={{ fontWeight: "bold", fontSize: "1.5em", marginBottom: "0px" }}
      >
        Developer
      </h2>
      <PanelSectionRow>
        <ButtonItem
          icon={<FaSteamSymbol style={{ display: "block" }} />}
          label="Sayo Kurisu"
          onClick={() => {
            Navigation.NavigateToExternalWeb(
              "https://steamcommunity.com/profiles/76561198217352855/"
            );
          }}
        >
          Steam Profile
        </ButtonItem>
      </PanelSectionRow>
      <h2
        style={{ fontWeight: "bold", fontSize: "1.5em", marginBottom: "0px" }}
      >
        Support
      </h2>
      <span>
        Join our Telegram group for support.
        <br />
      </span>
      <PanelSectionRow>
        <ButtonItem
          icon={<TbBrandTelegram style={{ display: "block" }} />}
          label="@steamdecktalk"
          onClick={() => {
            Navigation.NavigateToExternalWeb(
              "https://github.com/YukiCoco/StaticFilesCDN/blob/main/deck_tg_2.jpg?raw=true"
            );
          }}
        >
          Telegram Group
        </ButtonItem>
      </PanelSectionRow>
    </>
  );
};
