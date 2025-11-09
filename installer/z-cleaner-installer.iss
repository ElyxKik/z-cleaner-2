; Z-Cleaner Installation Wizard - Inno Setup Script
; Version 1.0 - Novembre 2025
; 
; Ce script crée un installeur professionnel pour Z-Cleaner
; Compatible avec Windows 7, 8, 10, 11 (32-bit et 64-bit)

#define MyAppName "Z-Cleaner"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "Elgrace Technologies"
#define MyAppURL "https://github.com/elykik/z-cleaner"
#define MyAppExeName "z-cleaner.exe"
#define MyAppDescription "Nettoyeur système professionnel multiplateforme"

; Chemin vers l'exécutable compilé par Tauri
#define SourceExePath "src-tauri\target\release\z-cleaner.exe"

[Setup]
; Informations de base
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
LicenseFile=installer\EULA_FR.txt
InfoBeforeFile=installer\WELCOME_FR.txt
OutputDir=installer\output
OutputBaseFilename=Z-Cleaner-Setup-{#MyAppVersion}
SetupIconFile=installer\icon.ico
WizardStyle=modern
WizardImageFile=installer\wizard-image.bmp
WizardSmallImageFile=installer\wizard-small-image.bmp

; Compression et optimisation
Compression=lzma2
SolidCompression=yes
CompressionThreads=auto

; Permissions et compatibilité
PrivilegesRequired=admin
PrivilegesRequiredOverridesAllowed=dialog
MinVersion=0,6.1
ArchitecturesInstallIn64BitMode=x64

; Langue
DefaultLanguage=French

[Languages]
Name: "french"; MessagesFile: "compiler:Languages\French.isl"
Name: "english"; MessagesFile: "compiler:Languages\English.isl"

; Pages personnalisées
[CustomMessages]
french.PrivacyPageTitle=Politique de Confidentialité
french.PrivacyPageSubtitle=Veuillez lire et accepter notre politique de confidentialité
french.PrivacyPageAccept=J'accepte la politique de confidentialité
english.PrivacyPageTitle=Privacy Policy
english.PrivacyPageSubtitle=Please read and accept our privacy policy
english.PrivacyPageAccept=I accept the privacy policy

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "quicklaunchicon"; Description: "{cm:CreateQuickLaunchIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked; OnlyBelowVersion: 6.1
Name: "startmenu"; Description: "Créer un raccourci dans le menu Démarrer"; GroupDescription: "{cm:AdditionalIcons}"; Flags: checked
Name: "associate"; Description: "Associer Z-Cleaner aux fichiers de configuration"; GroupDescription: "Options supplémentaires"; Flags: unchecked
Name: "privacy"; Description: "{cm:PrivacyPageAccept}"; GroupDescription: "Conditions d'utilisation"; Flags: checked

[Files]
; Fichier exécutable principal
Source: "{#SourceExePath}"; DestDir: "{app}"; Flags: ignoreversion
; Fichiers de support
Source: "installer\README.txt"; DestDir: "{app}"; Flags: ignoreversion isreadme
Source: "installer\LICENSE.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "installer\PRIVACY_POLICY_FR.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "installer\PRIVACY_POLICY_EN.txt"; DestDir: "{app}"; Flags: ignoreversion
; Icône de l'application
Source: "installer\icon.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
; Raccourci du menu Démarrer
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFileName: "{app}\icon.ico"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
; Raccourci du bureau
Name: "{commondesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon; IconFileName: "{app}\icon.ico"
; Barre de lancement rapide
Name: "{userappdata}\Microsoft\Internet Explorer\Quick Launch\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: quicklaunchicon; IconFileName: "{app}\icon.ico"

[Run]
; Lancer l'application après l'installation
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
; Supprimer les fichiers temporaires à la désinstallation
Type: filesandordirs; Name: "{app}"

[Code]
// Variables globales
var
  ProgressPage: TOutputProgressWizardPage;
  InstallationStarted: Boolean;
  PrivacyAccepted: Boolean;

// Procédure exécutée avant le début de l'installation
procedure InitializeWizard();
var
  PrivacyPage: TOutputMemoWizardPage;
  PrivacyText: String;
  PrivacyFile: String;
begin
  // Créer une page de progression personnalisée
  ProgressPage := CreateOutputProgressPage('Installation en cours', 'Veuillez patienter pendant l''installation de Z-Cleaner...');
  ProgressPage.SetProgress(0, 100);
  
  // Créer la page de politique de confidentialité
  if ActiveLanguage = 'french' then
    PrivacyFile := ExpandConstant('{src}\installer\PRIVACY_POLICY_FR.txt')
  else
    PrivacyFile := ExpandConstant('{src}\installer\PRIVACY_POLICY_EN.txt');
  
  // Charger le contenu du fichier de politique
  if FileExists(PrivacyFile) then
  begin
    if LoadStringFromFile(PrivacyFile, PrivacyText) then
    begin
      PrivacyPage := CreateOutputMemoPage(wpLicense, 
        ExpandConstant('{cm:PrivacyPageTitle}'),
        ExpandConstant('{cm:PrivacyPageSubtitle}'),
        PrivacyText);
      PrivacyPage.ReadOnly := True;
    end;
  end;
  
  PrivacyAccepted := False;
end;

// Vérifier les conditions avant l'installation
function NextButtonClick(CurPageID: Integer): Boolean;
begin
  Result := True;
  
  // Vérifier l'espace disque disponible
  if CurPageID = wpSelectDir then
  begin
    if GetDiskFreeSpace(ExpandConstant('{app}')) < 100 * 1024 * 1024 then
    begin
      MsgBox('Espace disque insuffisant. Au moins 100 MB d''espace libre est requis.', mbError, MB_OK);
      Result := False;
    end;
  end;
end;

// Vérifier les prérequis système
function InitializeSetup(): Boolean;
begin
  Result := True;
  
  // Vérifier la version de Windows
  if not (IsWin7 or IsWin8 or IsWin10 or IsWin11) then
  begin
    MsgBox('Z-Cleaner nécessite Windows 7 ou version ultérieure.', mbError, MB_OK);
    Result := False;
  end;
  
  // Vérifier les droits administrateur
  if not IsAdminLoggedOn then
  begin
    MsgBox('Les droits administrateur sont requis pour installer Z-Cleaner.', mbError, MB_OK);
    Result := False;
  end;
end;

// Mettre à jour la barre de progression
procedure CurInstallProgressChanged(CurProgress, MaxProgress: Integer);
begin
  if ProgressPage <> nil then
  begin
    ProgressPage.SetProgress(CurProgress, MaxProgress);
  end;
end;

// Procédure exécutée après l'installation
procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then
  begin
    // Créer des associations de fichiers si demandé
    if IsTaskSelected('associate') then
    begin
      // Ajouter les associations de fichiers au registre Windows
      RegWriteStringValue(HKEY_CLASSES_ROOT, '.zcfg', '', 'Z-Cleaner.Config');
      RegWriteStringValue(HKEY_CLASSES_ROOT, 'Z-Cleaner.Config', '', 'Z-Cleaner Configuration File');
      RegWriteStringValue(HKEY_CLASSES_ROOT, 'Z-Cleaner.Config\shell\open\command', '', ExpandConstant('"{app}\{#MyAppExeName}" "%%1"'));
    end;
  end;
end;

// Fonction pour vérifier si Windows 7 est installé
function IsWin7: Boolean;
begin
  Result := (GetWindowsVersion >= $06010000) and (GetWindowsVersion < $06020000);
end;

// Fonction pour vérifier si Windows 8 est installé
function IsWin8: Boolean;
begin
  Result := (GetWindowsVersion >= $06020000) and (GetWindowsVersion < $06030000);
end;

// Fonction pour vérifier si Windows 10 est installé
function IsWin10: Boolean;
begin
  Result := (GetWindowsVersion >= $06030000) and (GetWindowsVersion < $0A000000);
end;

// Fonction pour vérifier si Windows 11 est installé
function IsWin11: Boolean;
begin
  Result := GetWindowsVersion >= $0A000000;
end;

// Fonction pour obtenir l'espace disque disponible
function GetDiskFreeSpace(Path: String): Int64;
var
  FreeSpace: Int64;
begin
  if GetSpaceOnDisk(Path, True, FreeSpace, 0, 0) then
    Result := FreeSpace
  else
    Result := 0;
end;
