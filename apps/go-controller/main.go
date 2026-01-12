package main

import (
	"log"
	"math/rand"
	"net/http"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

var (
	// Custom metric: Shield Health Score
	shieldHealth = prometheus.NewGauge(prometheus.GaugeOpts{
		Name: "shield_system_health_score",
		Help: "Current health score of the Shield security system (0-100)",
	})

	// Custom metric: Active Scans
	activeScans = prometheus.NewGaugeVec(prometheus.GaugeOpts{
		Name: "shield_active_scans_total",
		Help: "Number of active verification scans by type",
	}, []string{"type"})
)

func init() {
	// Register metrics with Prometheus registry
	prometheus.MustRegister(shieldHealth)
	prometheus.MustRegister(activeScans)
}

func recordMetrics() {
	go func() {
		for {
			// Simulate health fluctuation
			// In a real app, this would query the Shield API/DB
			score := 95.0 + (rand.Float64() * 5.0) - (rand.Float64() * 2.0)
			shieldHealth.Set(score)

			// Simulate finding active scans
			activeScans.WithLabelValues("vulnerability").Set(float64(rand.Intn(5)))
			activeScans.WithLabelValues("compliance").Set(float64(rand.Intn(2)))

			log.Printf("Updated Shield metrics: Health=%.2f", score)
			time.Sleep(10 * time.Second)
		}
	}()
}

func main() {
	recordMetrics()

	http.Handle("/metrics", promhttp.Handler())
	http.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("OK"))
	})

	log.Println("Starting Go Control Plane on :8081")
	log.Fatal(http.ListenAndServe(":8081", nil))
}
