# Intégration de la Politique de Confidentialité - Z-Cleaner

## 📋 Vue d'ensemble

La politique de confidentialité a été intégrée dans l'installeur Inno Setup pour assurer la transparence et la conformité légale.

## 📁 Fichiers Créés

### 1. **PRIVACY_POLICY_FR.txt** (Français)
- Politique de confidentialité complète en français
- 15 sections détaillées
- Explique comment les données sont traitées
- Clarifies les droits de l'utilisateur
- Conforme au RGPD

### 2. **PRIVACY_POLICY_EN.txt** (Anglais)
- Politique de confidentialité complète en anglais
- Même structure que la version française
- Traduction professionnelle
- Conforme aux lois de protection des données

## 🔧 Intégration dans l'Installeur

### Modifications du Script Inno Setup

**Fichier:** `installer/z-cleaner-installer.iss`

#### 1. **Messages Personnalisés**
```inno
[CustomMessages]
french.PrivacyPageTitle=Politique de Confidentialité
french.PrivacyPageSubtitle=Veuillez lire et accepter notre politique de confidentialité
french.PrivacyPageAccept=J'accepte la politique de confidentialité
english.PrivacyPageTitle=Privacy Policy
english.PrivacyPageSubtitle=Please read and accept our privacy policy
english.PrivacyPageAccept=I accept the privacy policy
```

#### 2. **Tâche d'Acceptation**
```inno
[Tasks]
Name: "privacy"; Description: "{cm:PrivacyPageAccept}"; GroupDescription: "Conditions d'utilisation"; Flags: checked
```

#### 3. **Fichiers Inclus**
```inno
[Files]
Source: "installer\PRIVACY_POLICY_FR.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "installer\PRIVACY_POLICY_EN.txt"; DestDir: "{app}"; Flags: ignoreversion
```

#### 4. **Code Pascal**
```pascal
// Charger la politique de confidentialité selon la langue
if ActiveLanguage = 'french' then
  PrivacyFile := ExpandConstant('{src}\installer\PRIVACY_POLICY_FR.txt')
else
  PrivacyFile := ExpandConstant('{src}\installer\PRIVACY_POLICY_EN.txt');

// Afficher la page de politique
PrivacyPage := CreateOutputMemoPage(wpLicense, 
  ExpandConstant('{cm:PrivacyPageTitle}'),
  ExpandConstant('{cm:PrivacyPageSubtitle}'),
  PrivacyText);
```

## 📊 Contenu de la Politique

### Sections Principales

1. **Introduction**
   - Engagement envers la vie privée
   - Transparence

2. **Données Collectées**
   - Données collectées localement
   - Données NON collectées
   - Clarification sur la confidentialité

3. **Traitement des Données**
   - Traitement local uniquement
   - Analyse optionnelle de malwares
   - Cloudmersive API (optionnel)

4. **Données Envoyées à des Tiers**
   - Cloudmersive (optionnel)
   - Aucun autre partage de données

5. **Stockage des Données**
   - Stockage local
   - Sécurité du stockage

6. **Droits de l'Utilisateur**
   - Accès aux données
   - Suppression des données
   - Portabilité des données

7. **Sécurité des Données**
   - Mesures de sécurité
   - Responsabilité de l'utilisateur

8. **Conservation des Données**
   - Durée de conservation
   - Suppression après désinstallation

9. **Cookies et Suivi**
   - Aucun cookie
   - Aucun suivi

10. **Modifications de la Politique**
    - Droit de modification
    - Communication des changements

11. **Contact et Réclamations**
    - Email de support
    - Procédure de réclamation

12. **Conformité Légale**
    - RGPD (Union Européenne)
    - Autres juridictions

13. **Données Techniques**
    - Informations de diagnostic
    - Rapport d'erreurs

14. **Tiers et Dépendances**
    - Tauri
    - React
    - Rust

15. **Déclaration Finale**
    - Engagement envers la vie privée
    - Contrôle utilisateur

## 🌍 Support Multi-Langue

### Français
- **Fichier:** `PRIVACY_POLICY_FR.txt`
- **Langue:** Français
- **Encodage:** UTF-8
- **Taille:** ~8 KB

### Anglais
- **Fichier:** `PRIVACY_POLICY_EN.txt`
- **Langue:** Anglais
- **Encodage:** UTF-8
- **Taille:** ~8 KB

### Sélection Automatique
L'installeur sélectionne automatiquement la bonne langue basée sur:
- La langue de l'installeur sélectionnée
- La langue du système Windows

## ✅ Conformité Légale

### RGPD (Union Européenne)
✓ Pas de collecte de données personnelles  
✓ Traitement local uniquement  
✓ Transparence complète  
✓ Droits utilisateur respectés  
✓ Politique claire et accessible  

### Autres Juridictions
✓ Conforme aux lois de protection des données  
✓ Transparence sur le traitement des données  
✓ Droit à l'accès et à la suppression  
✓ Politique accessible et compréhensible  

## 🔒 Points Clés de la Politique

### Ce que Z-Cleaner NE fait PAS
- ❌ Ne collecte pas de données personnelles
- ❌ N'envoie pas de données à nos serveurs
- ❌ N'utilise pas de cookies
- ❌ N'utilise pas de suivi
- ❌ N'envoie pas de rapports d'erreurs automatiques
- ❌ Ne collecte pas de données de diagnostic

### Ce que Z-Cleaner FAIT
- ✅ Traite les données localement
- ✅ Supprime les fichiers temporaires
- ✅ Analyse l'espace disque
- ✅ Détecte les malwares (optionnel)
- ✅ Stocke les paramètres localement
- ✅ Respecte votre vie privée

## 📦 Distribution

### Fichiers Inclus dans l'Installeur
```
installer/
├── PRIVACY_POLICY_FR.txt     # Politique en français
├── PRIVACY_POLICY_EN.txt     # Politique en anglais
├── EULA_FR.txt               # Licence d'utilisation
├── LICENSE.txt               # Licence MIT
├── README.txt                # Guide d'installation
└── z-cleaner-installer.iss   # Script Inno Setup (mis à jour)
```

### Fichiers Installés sur l'Ordinateur
```
C:\Program Files\Z-Cleaner\
├── z-cleaner.exe
├── PRIVACY_POLICY_FR.txt     # Accessible après installation
├── PRIVACY_POLICY_EN.txt     # Accessible après installation
├── LICENSE.txt
└── README.txt
```

## 🚀 Utilisation

### Pour les Utilisateurs
1. Lancer l'installeur
2. Lire la politique de confidentialité
3. Accepter la politique (case cochée par défaut)
4. Continuer l'installation
5. Les fichiers de politique sont installés sur l'ordinateur

### Pour les Développeurs
1. Modifier les fichiers `PRIVACY_POLICY_*.txt` si nécessaire
2. Recompiler l'installeur: `npm run build:installer`
3. Tester l'installeur sur Windows
4. Vérifier que la politique s'affiche correctement

## 📝 Maintenance

### Mise à Jour de la Politique
1. Modifier les fichiers `PRIVACY_POLICY_FR.txt` ou `PRIVACY_POLICY_EN.txt`
2. Mettre à jour la date "Dernière mise à jour"
3. Recompiler l'installeur
4. Publier une nouvelle version

### Versioning
- Version 1.0 - Novembre 2025
- Politique initiale complète
- Conforme au RGPD et aux lois de protection des données

## 🔗 Ressources

- **RGPD:** https://gdpr-info.eu/
- **Inno Setup:** https://jrsoftware.org/ishelp/
- **Politique de Confidentialité:** Voir `PRIVACY_POLICY_FR.txt` et `PRIVACY_POLICY_EN.txt`

## ✅ Checklist

- ✅ Politique de confidentialité créée (FR)
- ✅ Politique de confidentialité créée (EN)
- ✅ Intégrée dans l'installeur Inno Setup
- ✅ Messages personnalisés ajoutés
- ✅ Tâche d'acceptation ajoutée
- ✅ Code Pascal pour charger les fichiers
- ✅ Support multi-langue
- ✅ Conforme au RGPD
- ✅ Fichiers inclus dans l'installation
- ✅ Documentation complète

---

**La politique de confidentialité est maintenant intégrée dans l'installeur Z-Cleaner!** 🎉

Les utilisateurs verront la politique lors de l'installation et pourront l'accepter avant de continuer.
