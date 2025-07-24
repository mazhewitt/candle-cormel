# candle-coreml Standalone Crate Project

## PURPOSE
Extract candle-coreml from the Candle monorepo into a standalone publishable crate on crates.io, providing CoreML inference capabilities for Candle users independently of the main project.

## PROJECT MOTIVATION
- Original PR to Candle was ignored
- Need freedom to take the API in our own direction
- Provide CoreML integration as an add-on that can be referenced from Candle documentation
- Give users access to CoreML inference without waiting for upstream integration

## CURRENT STATUS: ✅ COMPLETED - Independent Repository Created!

## ARCHITECTURAL DECISIONS
✅ DECIDED: Keep name as `candle-coreml`
- Describes exactly what it does (CoreML inference for Candle)
- Different from `coreml-rs` (generic swift-bridge bindings) 
- Our implementation uses objc2 directly with Candle-specific integration

## IMPLEMENTATION PLAN

### Phase 1: Research & Setup ✅ COMPLETED
1. ✅ Research current candle-coreml structure and dependencies
2. ✅ Research and decide on crate name (candle-coreml chosen)
3. ✅ Create new standalone crate structure with proper Cargo.toml
4. ✅ Extract and adapt candle-coreml code for standalone use

### Phase 2: Dependencies & Integration ✅ COMPLETED
5. ✅ Update dependencies to use published candle crates from crates.io
6. ✅ Move standalone crate outside of Candle repo and init as new git repo
7. ✅ Remove any remaining Candle workspace dependencies
8. ✅ Create comprehensive documentation and examples

### Phase 3: Publication Preparation
9. ⏳ Set up CI/CD for the standalone crate
10. ⏳ Prepare crate metadata for Cargo registry publication
11. ✅ Test standalone crate thoroughly before publication (8/8 tests pass)

## KEY DIFFERENTIATORS FROM coreml-rs

| Feature | coreml-rs | Our candle-coreml |
|---------|-----------|-------------------|
| Bindings | swift-bridge | objc2 direct |
| Purpose | Generic CoreML | Candle tensor integration |
| Scope | Raw CoreML bindings | Complete inference engine |
| Integration | None | Candle device validation, tensor conversion |
| Error Handling | Generic | Candle error types |
| API Style | Generic | Follows Candle patterns (like candle-metal) |

## CURRENT IMPLEMENTATION FEATURES
- CoreMLModel and CoreMLModelBuilder following T5-like patterns
- Device validation (accepts CPU/Metal, rejects CUDA)
- Comprehensive tensor conversion (F32/I64→I32 support)
- Full MLMultiArray ↔ Tensor conversion
- Integration tests with real .mlmodelc files
- Follows Candle conventions and error handling

## TECHNICAL APPROACH
- Extract existing candle-coreml code from monorepo
- Update Cargo.toml to use published candle crates from crates.io instead of workspace dependencies
- Maintain all existing functionality while making it standalone
- Preserve comprehensive test coverage
- Set up independent CI/CD pipeline

## SUCCESS CRITERIA
- [x] Crate builds and tests pass independently ✅ (8/8 tests pass)
- [ ] Published on crates.io
- [x] Comprehensive documentation and examples ✅ (README.md, examples/)
- [ ] CI/CD pipeline working
- [x] API can evolve independently from main Candle project ✅ (Independent git repo)
- [x] Users can easily integrate CoreML inference into their Candle projects ✅ (Ready to use)

## 🎉 MILESTONE ACHIEVED
**Independent candle-coreml repository successfully created!**

📍 **Location**: `/Users/mazdahewitt/projects/candle-coreml/`

## REPOSITORY STATUS
- ✅ Independent git repository initialized
- ✅ Initial commit with clean history
- ✅ Published Candle dependencies (candle-core 0.9.1)
- ✅ Comprehensive README and documentation
- ✅ All tests passing (integration and unit tests)
- ✅ Examples directory with working code
- ✅ Proper .gitignore and project structure

## NEXT PHASE: Publication Ready
Ready to continue with:
1. Setting up GitHub repository
2. Configuring CI/CD pipeline
3. Preparing crates.io publication metadata
4. Publishing to crates.io

The crate is now completely independent and ready for development!