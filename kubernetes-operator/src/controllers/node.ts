import { CustomObjectsApi, AppsV1Api, V1Deployment } from '@kubernetes/client-node';

export async function reconcilePhantomnetNode(
  customObjectsApi: CustomObjectsApi,
  appsApi: AppsV1Api
) {
  const ns = 'phantomnet', group = 'phantomnet.io', version = 'v1alpha1', plural = 'phantomnetnodes';
  const { body } = await customObjectsApi.listNamespacedCustomObject(group, version, ns, plural);
  for (const node of (body as any).items) {
    const name = node.metadata.name;
    const spec = node.spec;
    const deployment: V1Deployment = {
      metadata: { name, namespace: ns },
      spec: {
        replicas: spec.replicas,
        selector: { matchLabels: { app: name } },
        template: {
          metadata: { labels: { app: name, 'linkerd.io/inject': 'enabled' } },
          spec: {
            containers: [
              {
                name: 'nexa-node',
                image: spec.image,
                resources: spec.resources,
                env: [{ name: 'ENABLE_OTEL', value: String(!!spec.enableObservability) }]
              }
            ]
          }
        }
      }
    };
    try {
      await appsApi.readNamespacedDeployment(name, ns);
      await appsApi.replaceNamespacedDeployment(name, ns, deployment);
      console.log(`[PhantomNet] Updated deployment for node ${name}`);
    } catch (e) {
      await appsApi.createNamespacedDeployment(ns, deployment);
      console.log(`[PhantomNet] Created deployment for node ${name}`);
    }
  }
}