# Logo dans le Sidebar - Z-Cleaner

## 📋 Vue d'ensemble

Le logo Z-Cleaner a été intégré dans le sidebar de l'application, au-dessus du texte "Z-Cleaner".

## 🎨 Modifications Apportées

### Fichier: `src/App.tsx`

**Avant:**
```tsx
<div className="p-6 border-b border-slate-200 dark:border-slate-700">
  <h1 className="text-2xl font-bold bg-gradient-to-r from-blue-500 to-purple-600 bg-clip-text text-transparent">
    Z-Cleaner
  </h1>
  <p className="text-xs text-slate-500 dark:text-slate-400 mt-1">{t('sidebar.subtitle')}</p>
</div>
```

**Après:**
```tsx
<div className="p-6 border-b border-slate-200 dark:border-slate-700 flex flex-col items-center">
  {/* Logo */}
  <div className="mb-4 w-20 h-20 bg-gradient-to-br from-blue-400 to-blue-600 rounded-lg flex items-center justify-center shadow-lg">
    <img 
      src="/src-tauri/icons/icon.png" 
      alt="Z-Cleaner Logo" 
      className="w-16 h-16 object-contain"
    />
  </div>
  <h1 className="text-2xl font-bold bg-gradient-to-r from-blue-500 to-purple-600 bg-clip-text text-transparent text-center">
    Z-Cleaner
  </h1>
  <p className="text-xs text-slate-500 dark:text-slate-400 mt-1 text-center">{t('sidebar.subtitle')}</p>
</div>
```

## 🎯 Changements

### 1. **Conteneur Principal**
- Ajout de `flex flex-col items-center` pour centrer le contenu verticalement
- Le contenu est maintenant aligné au centre

### 2. **Logo**
- Conteneur bleu dégradé (20x20 pixels)
- Fond: `bg-gradient-to-br from-blue-400 to-blue-600`
- Ombre: `shadow-lg`
- Coins arrondis: `rounded-lg`
- Image du logo: `icon.png` (16x16 pixels)

### 3. **Texte**
- Ajout de `text-center` pour centrer le texte
- Le texte "Z-Cleaner" et le sous-titre sont maintenant centrés

## 📁 Ressources Utilisées

**Image du logo:**
```
src-tauri/icons/icon.png
```

Cette image est générée automatiquement par Tauri à partir du PNG source.

## 🎨 Styles Appliqués

### Logo Container
```css
w-20 h-20                              /* 80x80 pixels */
bg-gradient-to-br from-blue-400 to-blue-600  /* Dégradé bleu */
rounded-lg                             /* Coins arrondis */
flex items-center justify-center       /* Centrage */
shadow-lg                              /* Ombre */
mb-4                                   /* Marge inférieure */
```

### Logo Image
```css
w-16 h-16                              /* 64x64 pixels */
object-contain                         /* Préserver les proportions */
```

### Texte
```css
text-center                            /* Centrage horizontal */
```

## 🔄 Responsive Design

Le logo s'adapte automatiquement au mode clair/sombre grâce au dégradé bleu qui reste visible dans les deux modes.

### Mode Clair
- Fond du sidebar: `from-slate-100 to-slate-50`
- Logo: Dégradé bleu visible

### Mode Sombre
- Fond du sidebar: `from-slate-900 to-slate-950`
- Logo: Dégradé bleu visible

## 🖼️ Aperçu

```
┌─────────────────────────────────────┐
│                                     │
│         ┌─────────────────┐         │
│         │   [Logo Z]      │         │
│         └─────────────────┘         │
│                                     │
│         Z-Cleaner                   │
│    System Optimizer                 │
│                                     │
├─────────────────────────────────────┤
│                                     │
│  📊 Dashboard                       │
│  🔍 Analyzer                        │
│  🧹 Cleaner                         │
│  ⚙️  Optimizer                      │
│  ⚙️  Settings                       │
│                                     │
└─────────────────────────────────────┘
```

## 🔧 Personnalisation

### Modifier la Taille du Logo

Changez `w-20 h-20` et `w-16 h-16`:

```tsx
<div className="mb-4 w-24 h-24 bg-gradient-to-br from-blue-400 to-blue-600 rounded-lg flex items-center justify-center shadow-lg">
  <img 
    src="/src-tauri/icons/icon.png" 
    alt="Z-Cleaner Logo" 
    className="w-20 h-20 object-contain"  {/* Changé de w-16 h-16 */}
  />
</div>
```

### Modifier la Couleur du Fond

Changez `from-blue-400 to-blue-600`:

```tsx
<div className="mb-4 w-20 h-20 bg-gradient-to-br from-purple-400 to-purple-600 rounded-lg flex items-center justify-center shadow-lg">
  {/* ... */}
</div>
```

### Modifier l'Espacement

Changez `mb-4` (marge inférieure):

```tsx
<div className="mb-6 w-20 h-20 ...">  {/* Plus d'espace */}
  {/* ... */}
</div>
```

## ✅ Vérification

Pour vérifier que le logo s'affiche correctement:

1. Lancez l'application:
   ```bash
   npm run dev
   ```

2. Vérifiez que:
   - ✅ Le logo s'affiche dans le sidebar
   - ✅ Le logo est centré au-dessus du texte "Z-Cleaner"
   - ✅ Le logo est visible en mode clair et sombre
   - ✅ Le texte "Z-Cleaner" est centré sous le logo
   - ✅ Le sous-titre "System Optimizer" est centré

## 🐛 Dépannage

### Le logo ne s'affiche pas

**Solutions:**
1. Vérifiez que le fichier `src-tauri/icons/icon.png` existe
2. Régénérez les icônes:
   ```bash
   npm run icons:tauri
   ```
3. Recompiler l'app:
   ```bash
   npm run dev
   ```

### Le logo est déformé

**Solution:**
- Vérifiez que `object-contain` est appliqué à l'image
- Cela préserve les proportions du logo

### Le logo ne s'affiche pas en mode sombre

**Solution:**
- Le dégradé bleu devrait être visible dans les deux modes
- Si ce n'est pas le cas, changez la couleur du dégradé

## 📚 Ressources

- **Tailwind CSS**: https://tailwindcss.com/
- **Tailwind Gradient**: https://tailwindcss.com/docs/gradient-color-stops
- **Tailwind Flexbox**: https://tailwindcss.com/docs/display#flex

---

**Besoin d'aide ?** Consultez la documentation ou modifiez les classes Tailwind CSS dans `src/App.tsx`.
