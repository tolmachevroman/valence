Valence for macOS 13 or later on Apple Silicon.

**Install:** open the .dmg and drag Valence to Applications.

**First open:** Valence is a free showcase project and isn't notarized by Apple, so macOS blocks it the first time:

- macOS 15 and later: open Valence, close the warning, then go to System Settings → Privacy & Security and click **Open Anyway** next to the message about Valence.
- macOS 13 and 14: Control-click Valence in Applications, choose **Open**, then **Open** again.

Or, in Terminal: `xattr -dr com.apple.quarantine /Applications/Valence.app`

On first launch, Valence offers to download its AI model (2.5 GB, from Hugging Face). Search, 3D models and hazard labels work while it downloads.
