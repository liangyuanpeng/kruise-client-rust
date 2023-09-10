// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f -
// kopium version: 0.15.0

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::collections::HashMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "apps.kruise.io", version = "v1alpha1", kind = "UnitedDeployment", plural = "uniteddeployments")]
#[kube(namespaced)]
#[kube(status = "UnitedDeploymentStatus")]
#[kube(schema = "disabled")]
pub struct UnitedDeploymentSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revisionHistoryLimit")]
    pub revision_history_limit: Option<i32>,
    pub selector: UnitedDeploymentSelector,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<UnitedDeploymentTemplate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topology: Option<UnitedDeploymentTopology>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateStrategy")]
    pub update_strategy: Option<UnitedDeploymentUpdateStrategy>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<UnitedDeploymentSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "advancedStatefulSetTemplate")]
    pub advanced_stateful_set_template: Option<UnitedDeploymentTemplateAdvancedStatefulSetTemplate>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cloneSetTemplate")]
    pub clone_set_template: Option<UnitedDeploymentTemplateCloneSetTemplate>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deploymentTemplate")]
    pub deployment_template: Option<UnitedDeploymentTemplateDeploymentTemplate>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statefulSetTemplate")]
    pub stateful_set_template: Option<UnitedDeploymentTemplateStatefulSetTemplate>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub spec: UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpec,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecLifecycle>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistentVolumeClaimRetentionPolicy")]
    pub persistent_volume_claim_retention_policy: Option<UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecPersistentVolumeClaimRetentionPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podManagementPolicy")]
    pub pod_management_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reserveOrdinals")]
    pub reserve_ordinals: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revisionHistoryLimit")]
    pub revision_history_limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scaleStrategy")]
    pub scale_strategy: Option<UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecScaleStrategy>,
    pub selector: UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecSelector,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceName")]
    pub service_name: Option<String>,
    pub template: HashMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateStrategy")]
    pub update_strategy: Option<UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeClaimTemplates")]
    pub volume_claim_templates: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecLifecycle {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inPlaceUpdate")]
    pub in_place_update: Option<UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecLifecycleInPlaceUpdate>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preDelete")]
    pub pre_delete: Option<UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecLifecyclePreDelete>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preNormal")]
    pub pre_normal: Option<UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecLifecyclePreNormal>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecLifecycleInPlaceUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "finalizersHandler")]
    pub finalizers_handler: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelsHandler")]
    pub labels_handler: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "markPodNotReady")]
    pub mark_pod_not_ready: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecLifecyclePreDelete {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "finalizersHandler")]
    pub finalizers_handler: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelsHandler")]
    pub labels_handler: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "markPodNotReady")]
    pub mark_pod_not_ready: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecLifecyclePreNormal {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "finalizersHandler")]
    pub finalizers_handler: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelsHandler")]
    pub labels_handler: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "markPodNotReady")]
    pub mark_pod_not_ready: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecPersistentVolumeClaimRetentionPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "whenDeleted")]
    pub when_deleted: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "whenScaled")]
    pub when_scaled: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecScaleStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnavailable")]
    pub max_unavailable: Option<IntOrString>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rollingUpdate")]
    pub rolling_update: Option<UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdate>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inPlaceUpdateStrategy")]
    pub in_place_update_strategy: Option<UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdateInPlaceUpdateStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnavailable")]
    pub max_unavailable: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minReadySeconds")]
    pub min_ready_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podUpdatePolicy")]
    pub pod_update_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unorderedUpdate")]
    pub unordered_update: Option<UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdateUnorderedUpdate>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdateInPlaceUpdateStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gracePeriodSeconds")]
    pub grace_period_seconds: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdateUnorderedUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "priorityStrategy")]
    pub priority_strategy: Option<UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdateUnorderedUpdatePriorityStrategy>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdateUnorderedUpdatePriorityStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "orderPriority")]
    pub order_priority: Option<Vec<UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdateUnorderedUpdatePriorityStrategyOrderPriority>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "weightPriority")]
    pub weight_priority: Option<Vec<UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdateUnorderedUpdatePriorityStrategyWeightPriority>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdateUnorderedUpdatePriorityStrategyOrderPriority {
    #[serde(rename = "orderedKey")]
    pub ordered_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdateUnorderedUpdatePriorityStrategyWeightPriority {
    #[serde(rename = "matchSelector")]
    pub match_selector: UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdateUnorderedUpdatePriorityStrategyWeightPriorityMatchSelector,
    pub weight: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdateUnorderedUpdatePriorityStrategyWeightPriorityMatchSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdateUnorderedUpdatePriorityStrategyWeightPriorityMatchSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateAdvancedStatefulSetTemplateSpecUpdateStrategyRollingUpdateUnorderedUpdatePriorityStrategyWeightPriorityMatchSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub spec: UnitedDeploymentTemplateCloneSetTemplateSpec,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<UnitedDeploymentTemplateCloneSetTemplateSpecLifecycle>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minReadySeconds")]
    pub min_ready_seconds: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "revisionHistoryLimit")]
    pub revision_history_limit: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scaleStrategy")]
    pub scale_strategy: Option<UnitedDeploymentTemplateCloneSetTemplateSpecScaleStrategy>,
    pub selector: UnitedDeploymentTemplateCloneSetTemplateSpecSelector,
    pub template: HashMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateStrategy")]
    pub update_strategy: Option<UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "volumeClaimTemplates")]
    pub volume_claim_templates: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpecLifecycle {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inPlaceUpdate")]
    pub in_place_update: Option<UnitedDeploymentTemplateCloneSetTemplateSpecLifecycleInPlaceUpdate>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preDelete")]
    pub pre_delete: Option<UnitedDeploymentTemplateCloneSetTemplateSpecLifecyclePreDelete>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preNormal")]
    pub pre_normal: Option<UnitedDeploymentTemplateCloneSetTemplateSpecLifecyclePreNormal>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpecLifecycleInPlaceUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "finalizersHandler")]
    pub finalizers_handler: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelsHandler")]
    pub labels_handler: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "markPodNotReady")]
    pub mark_pod_not_ready: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpecLifecyclePreDelete {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "finalizersHandler")]
    pub finalizers_handler: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelsHandler")]
    pub labels_handler: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "markPodNotReady")]
    pub mark_pod_not_ready: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpecLifecyclePreNormal {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "finalizersHandler")]
    pub finalizers_handler: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelsHandler")]
    pub labels_handler: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "markPodNotReady")]
    pub mark_pod_not_ready: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpecScaleStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disablePVCReuse")]
    pub disable_pvc_reuse: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnavailable")]
    pub max_unavailable: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podsToDelete")]
    pub pods_to_delete: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpecSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<UnitedDeploymentTemplateCloneSetTemplateSpecSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpecSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inPlaceUpdateStrategy")]
    pub in_place_update_strategy: Option<UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategyInPlaceUpdateStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSurge")]
    pub max_surge: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnavailable")]
    pub max_unavailable: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "priorityStrategy")]
    pub priority_strategy: Option<UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategyPriorityStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scatterStrategy")]
    pub scatter_strategy: Option<Vec<UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategyScatterStrategy>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategyInPlaceUpdateStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gracePeriodSeconds")]
    pub grace_period_seconds: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategyPriorityStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "orderPriority")]
    pub order_priority: Option<Vec<UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategyPriorityStrategyOrderPriority>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "weightPriority")]
    pub weight_priority: Option<Vec<UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategyPriorityStrategyWeightPriority>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategyPriorityStrategyOrderPriority {
    #[serde(rename = "orderedKey")]
    pub ordered_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategyPriorityStrategyWeightPriority {
    #[serde(rename = "matchSelector")]
    pub match_selector: UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategyPriorityStrategyWeightPriorityMatchSelector,
    pub weight: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategyPriorityStrategyWeightPriorityMatchSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategyPriorityStrategyWeightPriorityMatchSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategyPriorityStrategyWeightPriorityMatchSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateCloneSetTemplateSpecUpdateStrategyScatterStrategy {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateDeploymentTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub spec: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTemplateStatefulSetTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub spec: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTopology {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subsets: Option<Vec<UnitedDeploymentTopologySubsets>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTopologySubsets {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelectorTerm")]
    pub node_selector_term: Option<UnitedDeploymentTopologySubsetsNodeSelectorTerm>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<UnitedDeploymentTopologySubsetsTolerations>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTopologySubsetsNodeSelectorTerm {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<UnitedDeploymentTopologySubsetsNodeSelectorTermMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<UnitedDeploymentTopologySubsetsNodeSelectorTermMatchFields>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTopologySubsetsNodeSelectorTermMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTopologySubsetsNodeSelectorTermMatchFields {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentTopologySubsetsTolerations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentUpdateStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "manualUpdate")]
    pub manual_update: Option<UnitedDeploymentUpdateStrategyManualUpdate>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentUpdateStrategyManualUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partitions: Option<BTreeMap<String, i32>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "collisionCount")]
    pub collision_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<UnitedDeploymentStatusConditions>>,
    #[serde(rename = "currentRevision")]
    pub current_revision: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readyReplicas")]
    pub ready_replicas: Option<i32>,
    pub replicas: i32,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subsetReplicas")]
    pub subset_replicas: Option<BTreeMap<String, i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateStatus")]
    pub update_status: Option<UnitedDeploymentStatusUpdateStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatedReadyReplicas")]
    pub updated_ready_replicas: Option<i32>,
    #[serde(rename = "updatedReplicas")]
    pub updated_replicas: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitedDeploymentStatusUpdateStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentPartitions")]
    pub current_partitions: Option<BTreeMap<String, i32>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatedRevision")]
    pub updated_revision: Option<String>,
}

