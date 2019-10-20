pip install --user --upgrade pip
pip install --user --upgrade awscli
ACCOUNT=391324319136
REGION=us-west-2
SECRET_NAME="$REGION-ecr-registry"
EMAIL=brennon.loveless@gmail.com
TOKEN=$(aws ecr get-login --region "$REGION" --registry-ids "$ACCOUNT" | cut -d' ' -f6)
echo "ENV variables setup done."
# javascript30 fritzandandre snippetbox mockrift-com
for NAMESPACE in $NAMESPACES
do
  echo "Working on namespace $NAMESPACE."
  kubectl -n $NAMESPACE delete secret --ignore-not-found $SECRET_NAME
  kubectl -n $NAMESPACE create secret docker-registry $SECRET_NAME \
    --docker-server=https://${ACCOUNT}.dkr.ecr.${REGION}.amazonaws.com \
    --docker-username=AWS \
    --docker-password="${TOKEN}" \
    --docker-email="${EMAIL}"
  echo "Secret created by name in namespace $NAMESPACE. $SECRET_NAME"
  kubectl -n $NAMESPACE patch serviceaccount default -p '{"imagePullSecrets":[{"name":"'$SECRET_NAME'"}]}'
done
echo "All done."
