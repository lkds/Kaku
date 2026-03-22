; Kaku Windows Installer Script (NSIS)
; Run with: makensis kaku.nsi

!define APP_NAME "Kaku"
!define APP_VERSION "0.8.0"
!define APP_PUBLISHER "lkds"
!define APP_URL "https://github.com/lkds/Kaku"
!define APP_EXE "kaku-gui.exe"
!define APP_GUID "a93b3b72-d338-5cc5-kaku-windows"

Name "${APP_NAME} ${APP_VERSION}"
OutFile "Kaku-${APP_VERSION}-windows-x64-setup.exe"
InstallDir "$PROGRAMFILES64\${APP_NAME}"
InstallDirRegKey HKLM "Software\${APP_NAME}" "Install_Dir"
RequestExecutionLevel admin
SetCompressor /SOLID lzma

; Modern UI
!include "MUI2.nsh"
!define MUI_ABORTWARNING
!define MUI_ICON "assets\kaku.ico"
!define MUI_UNICON "assets\kaku.ico"

; Pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "LICENSE.md"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"
!insertmacro MUI_LANGUAGE "SimpChinese"

Section "Kaku Terminal" SecMain
    SectionIn RO
    
    SetOutPath "$INSTDIR"
    
    ; Main executable
    File "target\release\${APP_EXE}"
    File "target\release\kaku.exe"
    
    ; Assets
    CreateDirectory "$INSTDIR\assets"
    File /r "assets\*.*"
    
    ; Config template
    CreateDirectory "$INSTDIR\config"
    File "config\*.*"
    
    ; Write uninstaller
    WriteUninstaller "$INSTDIR\Uninstall.exe"
    
    ; Registry
    WriteRegStr HKLM "Software\${APP_NAME}" "Install_Dir" "$INSTDIR"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "DisplayName" "${APP_NAME}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "UninstallString" '"$INSTDIR\Uninstall.exe"'
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "DisplayVersion" "${APP_VERSION}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "Publisher" "${APP_PUBLISHER}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "URLInfoAbout" "${APP_URL}"
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "NoModify" 1
    WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}" "NoRepair" 1
    
    ; Start Menu
    CreateDirectory "$SMPROGRAMS\${APP_NAME}"
    CreateShortcut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" "$INSTDIR\${APP_EXE}"
    CreateShortcut "$SMPROGRAMS\${APP_NAME}\Uninstall.lnk" "$INSTDIR\Uninstall.exe"
    
    ; Desktop shortcut
    CreateShortcut "$DESKTOP\${APP_NAME}.lnk" "$INSTDIR\${APP_EXE}"
    
SectionEnd

Section "Shell Integration" SecShell
    ; Right-click context menu for folders
    WriteRegStr HKCR "Directory\Background\shell\${APP_NAME}" "" "Open Kaku here"
    WriteRegStr HKCR "Directory\Background\shell\${APP_NAME}" "Icon" "$INSTDIR\${APP_EXE},0"
    WriteRegStr HKCR "Directory\Background\shell\${APP_NAME}\command" "" '"$INSTDIR\${APP_EXE}" --directory "%V"'
    
    WriteRegStr HKCR "Directory\shell\${APP_NAME}" "" "Open Kaku here"
    WriteRegStr HKCR "Directory\shell\${APP_NAME}" "Icon" "$INSTDIR\${APP_EXE},0"
    WriteRegStr HKCR "Directory\shell\${APP_NAME}\command" "" '"$INSTDIR\${APP_EXE}" --directory "%V"'
SectionEnd

Section "Uninstall"
    ; Remove files
    Delete "$INSTDIR\${APP_EXE}"
    Delete "$INSTDIR\kaku.exe"
    Delete "$INSTDIR\Uninstall.exe"
    RMDir /r "$INSTDIR\assets"
    RMDir /r "$INSTDIR\config"
    RMDir "$INSTDIR"
    
    ; Start Menu
    Delete "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk"
    Delete "$SMPROGRAMS\${APP_NAME}\Uninstall.lnk"
    RMDir "$SMPROGRAMS\${APP_NAME}"
    
    ; Desktop
    Delete "$DESKTOP\${APP_NAME}.lnk"
    
    ; Registry
    DeleteRegKey HKLM "Software\${APP_NAME}"
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APP_NAME}"
    
    ; Shell integration
    DeleteRegKey HKCR "Directory\Background\shell\${APP_NAME}"
    DeleteRegKey HKCR "Directory\shell\${APP_NAME}"
SectionEnd

; Lang strings
LangString DESC_SecMain ${LANG_ENGLISH} "Kaku Terminal main executable"
LangString DESC_SecShell ${LANG_ENGLISH} "Add 'Open Kaku here' to right-click menu"

!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
    !insertmacro MUI_DESCRIPTION_TEXT ${SecMain} $(DESC_SecMain)
    !insertmacro MUI_DESCRIPTION_TEXT ${SecShell} $(DESC_SecShell)
!insertmacro MUI_FUNCTION_DESCRIPTION_END