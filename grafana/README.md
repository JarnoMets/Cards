# Grafana Dashboard for Card Game Scorer

This folder contains Grafana provisioning files to visualize card game statistics directly from the PostgreSQL database.

## Contents

- `dashboards/card-game-scorer.json` - Main dashboard JSON
- `provisioning/datasources/postgres.yaml` - PostgreSQL datasource configuration
- `provisioning/dashboards/dashboards.yaml` - Dashboard provisioning configuration

## Dashboard Features

The dashboard shows:

### Overview
- Total games played
- Unique players
- Total rounds played
- Games this week

### Games Over Time
- Games by type (stacked bar chart, last 30 days)
- Game type distribution (pie chart)
- Unique players per day

### Top Players
- Most active players (by games played)
- Top scores by game type
- Best average score (minimum 3 games)

### Recent Games
- List of recent games with players and winner

## Deployment

### Option 1: Mount as volume in Grafana

Mount the provisioning files into your Grafana container:

```yaml
volumes:
  - name: grafana-dashboards-provisioning
    configMap:
      name: grafana-dashboards-provisioning
  - name: grafana-dashboards
    configMap:
      name: grafana-dashboard-card-game-scorer
  - name: grafana-datasources-postgres
    configMap:
      name: grafana-datasource-postgres-cards

volumeMounts:
  - name: grafana-dashboards-provisioning
    mountPath: /etc/grafana/provisioning/dashboards
  - name: grafana-dashboards
    mountPath: /var/lib/grafana/dashboards
  - name: grafana-datasources-postgres
    mountPath: /etc/grafana/provisioning/datasources/postgres-cards.yaml
    subPath: postgres-cards.yaml
```

### Option 2: Apply the K8s ConfigMaps

```bash
kubectl apply -f k8s/grafana-datasource-postgres.yaml
```

Then add the dashboard JSON as another ConfigMap or import it manually via the Grafana UI.

### Option 3: Import manually

1. In Grafana, go to **Connections** → **Data sources** → **Add data source**
2. Select **PostgreSQL** and configure:
   - Host: `postgres.postgres.svc.cluster.local:5432`
   - Database: `postgresdb`
   - User: `postgresuser`
   - Password: `postgrespwd`
   - SSL Mode: `disable`
3. Go to **Dashboards** → **Import**
4. Upload or paste the contents of `dashboards/card-game-scorer.json`
