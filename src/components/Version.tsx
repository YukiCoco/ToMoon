import { PanelSection, PanelSectionRow, Field } from "@decky/ui";
import { FC, useEffect, useState } from "react";
import { PyBackend } from "../backend/backend";
import { ActionButtonItem } from ".";
import { localizationManager, localizeStrEnum } from "../i18n";

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

  let uptButtonText = localizationManager.getString(
    localizeStrEnum.REINSTALL_PLUGIN
  );

  if (currentVersion !== latestVersion && Boolean(latestVersion)) {
    uptButtonText =
      localizationManager.getString(localizeStrEnum.UPDATE_TO) +
      ` ${latestVersion}`;
  }

  return (
    <PanelSection
      title={localizationManager.getString(localizeStrEnum.VERSION)}
    >
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
          label={localizationManager.getString(
            localizeStrEnum.INSTALLED_VERSION
          )}
        >
          {currentVersion}
        </Field>
      </PanelSectionRow>
      {Boolean(latestVersion) && (
        <PanelSectionRow>
          <Field
            focusable
            label={localizationManager.getString(
              localizeStrEnum.LATEST_VERSION
            )}
          >
            {latestVersion}
          </Field>
        </PanelSectionRow>
      )}
    </PanelSection>
  );
};
