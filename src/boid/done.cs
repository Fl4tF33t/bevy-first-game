using System.Collections.Generic;
using UnityEngine;
using UnityUtils;

[DisallowMultipleComponent]
public class BoidAgent : MonoBehaviour {
    #region Fields
    
    [Header("Movement")]
    [SerializeField, Min(0.1f)] float maxSpeed = 7f;
    [SerializeField, Min(0.1f)] float accelerationForce = 18f;
    [SerializeField, Min(0.1f)] float rotationSharpness = 10f;
    
    [Header("Behavior Distances")]
    [SerializeField, Min(0.1f)] float separationDistance = 1.5f;
    [SerializeField, Min(0.1f)] float alignmentDistance = 3.5f;
    [SerializeField, Min(0.1f)] float cohesionDistance = 4.5f;
    
    [Header("Behavior Weights")]
    [SerializeField, Min(0f)] float separationWeight = 1.6f;
    [SerializeField, Min(0f)] float alignmentWeight = 1f;
    [SerializeField, Min(0f)] float cohesionWeight = 1.2f;
    [SerializeField, Min(0f)] float boundsWeight = 2.5f;
    [SerializeField, Min(0f)] float obstacleWeight = 3f;
    
    [Header("Neighbor Query")]
    [SerializeField] LayerMask neighborMask = ~0;
    [SerializeField, Min(8)] int maxNeighborColliders = 128;
    
    [Header("Obstacle Avoidance")]
    [SerializeField] bool avoidObstacles = true;
    [SerializeField] LayerMask obstacleMask = ~0;
    [SerializeField, Min(0.1f)] float obstacleProbeRadius = 0.45f;
    [SerializeField, Min(0.5f)] float obstacleLookAhead = 3.5f;
    [SerializeField, Min(0.1f)] float floorClearance = 0.8f;
    
    [Header("Vertical Constraint")]
    [SerializeField] bool constrainY;
    [SerializeField] float constrainedY = 4f;
    
    Rigidbody rb;
    readonly List<BoidAgent> neighbors = new(32);
    Collider[] neighborHits;
    
    Vector3 boundsCenter;
    float boundsRadius = 25f;
    bool useBounds = true;
    
    float separationDistanceSqr;
    float alignmentDistanceSqr;
    float cohesionDistanceSqr;
    float neighborScanRadius;

    #endregion
    
    public Vector3 Velocity => rb ? rb.linearVelocity : Vector3.zero;

    protected void Awake() {
        rb = gameObject.GetOrAdd<Rigidbody>();
        rb.useGravity = false;
        rb.constraints = RigidbodyConstraints.FreezeRotation;
        rb.interpolation = RigidbodyInterpolation.Interpolate;
        rb.collisionDetectionMode = CollisionDetectionMode.ContinuousSpeculative;
        rb.linearDamping = 0.2f;
        
        var bufferSize = Mathf.Max(8, maxNeighborColliders);
        neighborHits = new Collider[bufferSize];
        
        separationDistanceSqr = separationDistance * separationDistance;
        alignmentDistanceSqr = alignmentDistance * alignmentDistance;
        cohesionDistanceSqr = cohesionDistance * cohesionDistance;
        neighborScanRadius = Mathf.Max(separationDistance, Mathf.Max(alignmentDistance, cohesionDistance));
    }

    protected void Start() {
        if (rb.linearVelocity.sqrMagnitude < 0.01f) rb.linearVelocity = Random.onUnitSphere * maxSpeed;
        if (!constrainY) return;
        
        var position = transform.position;
        position.y = constrainedY;
        transform.position = position;
    }

    public void ConfigureBounds(Vector3 center, float radius, bool enabled = true) {
        boundsCenter = center;
        boundsRadius = Mathf.Max(0.5f, radius);
        useBounds = enabled;
    }
    public void ConfigureSpeed(float speed) => maxSpeed = Mathf.Max(0.1f, speed);
    public void ConfigureVerticalConstraint(bool enabled, float yValue) {
        constrainY = enabled;
        constrainedY = yValue;
    }

    void FindNeighbors() {
        neighbors.Clear();
        var hitCount = Physics.OverlapSphereNonAlloc(
            transform.position,
            neighborScanRadius,
            neighborHits,
            neighborMask,
            QueryTriggerInteraction.Collide
        );

        for (var i = 0; i < hitCount; i++) {
            var hit = neighborHits[i];
            if (!hit) continue;
            
            var other = hit.attachedRigidbody ? hit.attachedRigidbody.GetComponent<BoidAgent>() : hit.GetComponent<BoidAgent>();
            if (!other || other == this) continue;
            
            neighbors.Add(other);
        }
    }

    Vector3 ComputeSeparation() {
        var force = Vector3.zero;
        var count = 0;
        var position = transform.position;

        for (var i = 0; i < neighbors.Count; i++) {
            var toOther = position - neighbors[i].transform.position;
            var sqrDistance = toOther.sqrMagnitude;
            if (sqrDistance > separationDistanceSqr || sqrDistance < 0.0001f) continue;
            
            force += toOther / sqrDistance;
            count++;
        }
        
        return count > 0 ? force / count : Vector3.zero;
    }
    Vector3 ComputeAlignment() {
        var averageVelocity = Vector3.zero;
        var count = 0;
        var position = transform.position;

        for (var i = 0; i < neighbors.Count; i++) {
            var offset = neighbors[i].transform.position - position;
            if (offset.sqrMagnitude > alignmentDistanceSqr) continue;
            
            averageVelocity += neighbors[i].Velocity;
            count++;
        }
        
        return count > 0 ? (averageVelocity / count).normalized : transform.forward;
    }
    Vector3 ComputeCohesion() {
        var center = Vector3.zero;
        var count = 0;
        var position = transform.position;

        for (var i = 0; i < neighbors.Count; i++) {
            var otherPosition = neighbors[i].transform.position;
            if ((otherPosition - position).sqrMagnitude > cohesionDistanceSqr) continue;
            
            center += otherPosition;
            count++;
        }
        
        return count > 0 ? (center / count - position).normalized : Vector3.zero;
    }
    Vector3 ComputeBoundsSteer() {
        if (!useBounds) return Vector3.zero;
        
        var offset = transform.position - boundsCenter;
        var distance = offset.magnitude;
        var innerRadius = boundsRadius * 0.85f;
        if (distance <= innerRadius) return Vector3.zero;
        
        var strength = Mathf.InverseLerp(innerRadius, boundsRadius, distance);
        return (boundsCenter - transform.position).normalized * strength;
    }
    Vector3 ComputeObstacleAvoidance() {
        if (!avoidObstacles) return Vector3.zero;
        
        var velocity = rb.linearVelocity;
        if (velocity.sqrMagnitude < 0.0001f) return Vector3.zero;
        
        var direction = velocity.normalized;
        var position = transform.position;
        var avoidance = Vector3.zero;

        if (Physics.SphereCast(position, obstacleProbeRadius, direction, out var forwardHit, obstacleLookAhead, obstacleMask, QueryTriggerInteraction.Ignore)) {
            var awayFromHit = Vector3.Reflect(direction, forwardHit.normal).normalized;
            avoidance += awayFromHit;
        }

        if (Physics.Raycast(position, Vector3.down, out var floorHit, floorClearance, obstacleMask, QueryTriggerInteraction.Ignore)) {
            var floorStrength = Mathf.InverseLerp(floorClearance, 0f, floorHit.distance);
            avoidance += Vector3.up * floorStrength;
        }
        
        return avoidance.normalized;
    }

    protected void FixedUpdate() {
        FindNeighbors();
        
        // This one block is the boids thesis: blend local influences and let emergence do the rest.
        var steering =
            ComputeSeparation() * separationWeight +
            ComputeAlignment() * alignmentWeight +
            ComputeCohesion() * cohesionWeight+
            ComputeBoundsSteer() * boundsWeight+
            ComputeObstacleAvoidance() * obstacleWeight;
        
        if (steering.sqrMagnitude < 0.0001f) steering = transform.forward;
        var acceleration = steering.normalized * accelerationForce;
        var nextVelocity = rb.linearVelocity + acceleration * Time.fixedDeltaTime;
        if (constrainY) nextVelocity.y = 0f;
        rb.linearVelocity = Vector3.ClampMagnitude(nextVelocity, maxSpeed);

        if (rb.linearVelocity.sqrMagnitude > 0.01f) {
            var lookDirection = Vector3.ProjectOnPlane(rb.linearVelocity, Vector3.up);
            var targetRotation = Quaternion.LookRotation(lookDirection.normalized, Vector3.up);
            transform.rotation = Quaternion.Slerp(transform.rotation, targetRotation, rotationSharpness * Time.fixedDeltaTime);
        }
    }
}