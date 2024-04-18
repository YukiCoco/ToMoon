import { PanelSection, PanelSectionRow, Field } from "decky-frontend-lib";
import { VFC, useEffect, useState } from "react";
import { PyBackend } from "../backend";
import { ActionButtonItem } from "./components/actionButtonItem";


export const VersionComponent: VFC = () => {
  const [currentVersion, _] = useState<string>(PyBackend.data.getCurrentVersion());
  const [latestVersion, setLatestVersion] = useState<string>(PyBackend.data.getLatestVersion());

  useEffect(() => {
    const getData = async () => {
      const latestVersion = await PyBackend.getLatestVersion();
      setLatestVersion(latestVersion);
      PyBackend.data.setLatestVersion(latestVersion);
    };
    getData();
  });

  let uptButtonText = 'Reinstall Plugin';

  if (currentVersion !== latestVersion && Boolean(latestVersion)) {
    uptButtonText = `Update to ${latestVersion}`;
  }

  return (
    <PanelSection title={'Version'}>
      <PanelSectionRow>
        <ActionButtonItem
          layout="below"
          onClick={async () => {
            await PyBackend.updateLatest();
          }}
        >{uptButtonText}</ActionButtonItem>
      </PanelSectionRow>
      <PanelSectionRow>
        <Field focusable label={'Installed Version'}>
          {currentVersion}
        </Field>
      </PanelSectionRow>
      {Boolean(latestVersion) && (
        <PanelSectionRow>
          <Field focusable label={'Latest Version'}>
            {latestVersion}
          </Field>
        </PanelSectionRow>
      )}
    </PanelSection>
  )
}