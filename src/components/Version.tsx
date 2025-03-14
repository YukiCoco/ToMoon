import { PanelSection, PanelSectionRow, Field } from "@decky/ui";
import { FC, useEffect, useState } from "react";
import { PyBackend } from "../backend/backend";
import { ActionButtonItem } from ".";
import { localizationManager, L } from "../i18n";

export const VersionComponent: FC = () => {
  const [currentVersion, _] = useState<string>(
    PyBackend.data.getCurrentVersion()
  );
  const [latestVersion, setLatestVersion] = useState<string>(
    PyBackend.data.getLatestVersion()
  );

  useEffect(() => {
    const getData = async () => {
      const latestVersion = await PyBackend.getLatestVersion();
      setLatestVersion(latestVersion);
      PyBackend.data.setLatestVersion(latestVersion);
    };
    getData();
  });

  let uptButtonText = localizationManager.getString(L.REINSTALL_PLUGIN);

  if (currentVersion !== latestVersion && Boolean(latestVersion)) {
    uptButtonText =
      localizationManager.getString(L.UPDATE_TO) + ` ${latestVersion}`;
  }

  return (
    <PanelSection title={localizationManager.getString(L.VERSION)}>
      <PanelSectionRow>
        <ActionButtonItem
          layout="below"
          onClick={async () => {
            await PyBackend.updateLatest();
          }}
        >
          {uptButtonText}
        </ActionButtonItem>
      </PanelSectionRow>
      <PanelSectionRow>
        <Field
          focusable
          label={localizationManager.getString(L.INSTALLED_VERSION)}
        >
          {currentVersion}
        </Field>
      </PanelSectionRow>
      {Boolean(latestVersion) && (
        <PanelSectionRow>
          <Field
            focusable
            label={localizationManager.getString(L.LATEST_VERSION)}
          >
            {latestVersion}
          </Field>
        </PanelSectionRow>
      )}
    </PanelSection>
  );
};
