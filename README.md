# Rusty Rhythm 🎵🦀

A rhythm game built in Rust using the [Bevy](https://bevyengine.org/) game engine. Inspired by classic rhythm games like StepMania, but with modern mechanics and smooth Rust performance.

## 🎮 Game Concept

Rusty Rhythm aims to capture the essence of classic 4-key rhythm games while adding unique features and smooth gameplay. Players hit notes in time with music across various difficulty levels.

### Core Features (Planned)
- **4-Key Gameplay**: Classic VSRG (Vertical Scrolling Rhythm Game) mechanics
- **Custom Song Support**: Import your own music and charts
- **Multiple Difficulty Levels**: From beginner to expert charts
- **Smooth 60+ FPS Gameplay**: Leveraging Bevy's performance
- **Customizable Skins**: Personalize your play experience
- **Multiplayer Support**: Play with friends online
- **Chart Editor**: Create and share your own beatmaps

## 🚀 Technology Stack

- **Engine**: [Bevy](https://bevyengine.org/) - A refreshingly simple data-driven game engine
- **Language**: Rust - For memory safety and performance
- **Audio**: `bevy_kira_audio` - High-quality audio playback
- **UI**: Bevy's built-in UI system with custom components

## 🎯 Game Inspiration & Similar Games

### Primary Inspiration
- **[StepMania](https://www.stepmania.com/)** - The gold standard for rhythm games
- **[osu!mania](https://osu.ppy.sh/)** - Modern competitive rhythm gaming
- **[Quaver](https://quavergame.com/)** - Community-driven VSRG

### Potential Game Modes
We're considering multiple game modes to keep things interesting:

1. **Classic 4K**: Traditional 4-key vertical scrolling
2. **7K Mode**: More complex 7-key gameplay
3. **Speed Mode**: Accelerating note speeds for extra challenge
4. **Mirror Mode**: Horizontally flipped note patterns
5. **Random Mode**: Randomized note lanes for muscle memory breaking
6. **Co-op Mode**: Team up with friends on the same song

### Unique Features We Want to Add
- **Dynamic Difficulty**: AI-assisted difficulty scaling based on performance
- **Visual Effects**: Reactive backgrounds that pulse with the music
- **Community Charts**: Easy sharing and rating system for user-created content
- **Practice Mode**: Slow-down and section practice features
- **Performance Analytics**: Detailed statistics and improvement tracking

## 🛠️ Development Setup

### Prerequisites
- Rust (latest stable version)
- Git

### Getting Started
```bash
# Clone the repository
git clone https://github.com/tora-pan/rusty_rhythm.git
cd rusty_rhythm

# Run the game in development mode
cargo run

# Run tests
cargo test

# Build for release
cargo build --release
```

### Project Structure
```
rusty_rhythm/
├── src/
│   ├── main.rs              # Entry point
│   ├── game/                # Core game logic
│   ├── audio/               # Audio processing and playback
│   ├── ui/                  # User interface components
│   ├── input/               # Input handling and keybinds
│   ├── charts/              # Chart loading and parsing
│   └── networking/          # Multiplayer functionality
├── assets/                  # Game assets (music, images, etc.)
├── charts/                  # Beatmap files
└── docs/                    # Documentation
```

## 🎵 Chart Format

We plan to support multiple chart formats:
- **Custom JSON format** - For new charts created in our editor
- **StepMania .sm files** - For compatibility with existing charts
- **osu!mania .osu files** - Broader chart library support

## 🤝 Contributing

This is a collaborative project between friends! We welcome contributions in the form of:

- **Code improvements** and bug fixes
- **Chart creation** and testing
- **Art and visual design** contributions
- **Music composition** for original tracks
- **Documentation** improvements
- **Testing** and feedback

### Development Guidelines
- Follow Rust best practices and conventions
- Write tests for new features
- Update documentation for API changes
- Use descriptive commit messages

## 🎨 Art & Audio Assets

### Music
- Original compositions welcome!
- Ensure you have proper licensing for any copyrighted music
- Recommended formats: OGG, MP3, WAV

### Visual Assets
- UI elements should follow the established design system
- Animations should be smooth and not distracting during gameplay
- Consider accessibility (colorblind-friendly palettes)

## 🗺️ Roadmap

### Phase 1: Foundation
- [ ] Basic Bevy project setup
- [ ] Simple note rendering and scrolling
- [ ] Basic input detection
- [ ] Audio playback integration

### Phase 2: Core Gameplay
- [ ] Note hitting mechanics and scoring
- [ ] Chart loading system
- [ ] Basic UI (menus, song selection)
- [ ] Performance tracking

### Phase 3: Polish & Features
- [ ] Customizable skins and themes
- [ ] Chart editor
- [ ] Practice mode features
- [ ] Enhanced visual effects

### Phase 4: Community
- [ ] Multiplayer functionality
- [ ] Chart sharing system
- [ ] Leaderboards and profiles
- [ ] Tournament mode

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The [Bevy community](https://github.com/bevyengine/bevy) for the amazing game engine
- [StepMania](https://www.stepmania.com/) for pioneering the VSRG genre
- The rhythm game community for inspiration and feedback

## 📞 Contact
- **Issues**: Please use GitHub Issues for bug reports and feature requests

---

*Let's make some rhythm magic happen! 🎵✨*