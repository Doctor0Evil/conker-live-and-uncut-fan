#pragma once

#include "CoreMinimal.h"
#include "GameFramework/PlayerController.h"
#include "CLUPlayerController.generated.h"

// SystemNode ID: systems.ue5.conker.core.playercontroller
// Tags: InputRouting, CameraManagement, UI, Deterministic
// Dependencies: systems.ue5.conker.core.character, schemas.input_profile.json

UCLASS()
class CONKERLIVEUNCUT_API ACLUPlayerController : public APlayerController
{
	GENERATED_BODY()

public:
	ACLUPlayerController(const FObjectInitializer& ObjectInitializer);

protected:
	virtual void BeginPlay() override;

	// Camera setup components
	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Camera", meta = (AllowPrivateAccess = "true"))
	class USpringArmComponent* CameraBoom;

	UPROPERTY(VisibleAnywhere, BlueprintReadOnly, Category = "Camera", meta = (AllowPrivateAccess = "true"))
	class UCameraComponent* FollowCamera;

	// Input parameters
	UPROPERTY(EditDefaultsOnly, Category = "Camera")
	float BaseTurnRate = 45.0f;
	
	UPROPERTY(EditDefaultsOnly, Category = "Camera")
	float BaseLookUpRate = 45.0f;

	UPROPERTY(EditDefaultsOnly, Category = "Camera")
	float ArmLength = 300.0f;

	// HUD/UI references
	UPROPERTY(Transient)
	TWeakObjectPtr<class UCLUHUD> HUDReference;

public:
	// Input handlers
	UFUNCTION()
	void Turn(float Amount);
	
	UFUNCTION()
	void LookUp(float Amount);

	// UI routing and interaction
	UFUNCTION(BlueprintCallable, Category = "UI")
	void ToggleInventory();

	UFUNCTION(BlueprintCallable, Category = "UI")
	void OpenMap();

	// Possession hook to attach camera to new character
	virtual void OnPossess(APawn* NewPawn) override;

	// Determinism validation for input sequences
	UFUNCTION(BlueprintCallable, Category = "Determinism")
	uint32 ComputeInputHash(const TArray<FKey>& PressedKeys) const;

protected:
	// Setup camera components
	void SetupCamera();

private:
	// Internal state for deterministic input tracking
	uint32 LastInputHash;
};
